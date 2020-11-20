package main

import (
	"context"
	"encoding/json"
	"flag"
	"fmt"
	"github.com/care0717/pro_con/cmd/atcli/model"
	"github.com/pkg/errors"
	"io"
	"io/ioutil"
	"log"
	"os"
	"os/exec"
	"path"
	"strings"
	"sync"
	"time"

	"net/http"
	"net/http/cookiejar"
)

func main() {
	flag.Usage = func() {
		usageTxt := `Usage atcoder command [args]
	Usage:
		atcoder command [arguments]
	The commands are:
		create    fetch contest info and create from template
		test      test 
Use "atcoder [command] --help" for more infomation about a command`
		fmt.Fprintf(os.Stderr, "%s\n", usageTxt)
	}

	if len(os.Args) == 1 {
		flag.Usage()
		return
	}
	switch os.Args[1] {
	case "create":
		if len(os.Args) <= 2 {
			log.Fatal("please input contest name. like `abc163`")
			return
		}
		contestName := os.Args[2]
		if err := create(contestName); err != nil {
			log.Fatal(err)
			return
		}

	case "test":
		if len(os.Args) <= 3 {
			log.Fatal("please input contest and problem name. like `abc163 A`")
			return
		}
		contestName := os.Args[2]
		problemName := os.Args[3]
		if err := test(contestName, problemName); err != nil {
			log.Fatal(err)
			return
		}

	default:
		flag.Usage()
	}
}

func test(contestName, problemName string) error {
	c := model.NewContest(contestName)
	subdir := problemName
	bytes, err := ioutil.ReadFile(path.Join(c.DirName(), subdir, fmt.Sprintf("%s.json", problemName)))
	if err != nil {
		return err
	}
	var samples []model.Sample
	if err := json.Unmarshal(bytes, &samples); err != nil {
		return err
	}
	targetFilePath := path.Join(c.DirName(), subdir, fmt.Sprintf("%s.go", problemName))
	execPath := path.Join("/tmp", problemName)
	cmd := exec.Command("go", "build", "-o", execPath, targetFilePath)
	stderr, err := cmd.StderrPipe()
	if err != nil {
		return err
	}
	if err := cmd.Start(); err != nil {
		return err
	}
	slurp, _ := ioutil.ReadAll(stderr)
	if len(slurp) != 0 {
		return fmt.Errorf("failed go build.\n %s", slurp)
	}
	if err := cmd.Wait(); err != nil {
		return err
	}

	defer os.Remove(execPath)
	for i, s := range samples {
		timeoutLimit := 2 * time.Second
		ctx, cancel := context.WithTimeout(context.Background(), timeoutLimit)
		cmd := exec.CommandContext(ctx, execPath)
		stdin, err := cmd.StdinPipe()
		if err != nil {
			return err
		}

		go func() {
			defer stdin.Close()
			io.WriteString(stdin, s.Input)
		}()
		stderr, err := cmd.StderrPipe()
		if err != nil {
			return err
		}
		stdout, err := cmd.StdoutPipe()
		if err != nil {
			return err
		}
		if err := cmd.Start(); err != nil {
			return err
		}

		slurp, _ := ioutil.ReadAll(stderr)
		if len(slurp) != 0 {
			return errors.New(string(slurp))
		}
		out, _ := ioutil.ReadAll(stdout)

		if err := cmd.Wait(); err != nil {
			if errors.Is(ctx.Err(), context.DeadlineExceeded) {
				err = errors.Wrap(err, fmt.Sprintf("In case %d, context deadline exceeded (over %s)", i, timeoutLimit.String()))
			} else if ctx.Err() != nil {
				err = errors.Wrap(err, ctx.Err().Error())
			}
			return err
		}
		actual := strings.TrimRight(string(out), "\n")
		if actual == s.Output {
			fmt.Printf("case %d OK\n", i)
		} else {
			fmt.Printf("case %d NG. expect %s, but got %s\n", i, s.Output, actual)
		}
		cancel()
	}
	return nil
}

func create(contestName string) error {
	c := model.NewContest(contestName)
	if c.ExistDir() {
		return fmt.Errorf("directory already exists. %s", c.DirName())
	}

	cookieJar, _ := cookiejar.New(nil)
	client := &http.Client{
		Jar: cookieJar,
	}
	username := os.Getenv("ATCODER_USERNAME")
	password := os.Getenv("ATCODER_PASSWORD")
	if err := model.Login(client, username, password); err != nil {
		return err
	}
	log.Printf("Login as %s", username)

	if !c.ValidContest(client) {
		return fmt.Errorf("invalid contest name: %s", contestName)
	}
	if err := c.CreateDir(); err != nil {
		return err
	}
	log.Printf("Create dir %s", c.DirName())

	hrefs, err := c.FetchProblemHrefs(client)
	if err != nil {
		return err
	}
	wg := &sync.WaitGroup{}
	for _, h := range hrefs {
		wg.Add(1)
		go func(h string) {
			defer wg.Done()
			p := model.NewProblem(h)
			samples, err := p.FetchSamples(client)
			log.Printf("Fetch %s", h)
			bytes, err := json.MarshalIndent(samples, "", " ")
			if err != nil {
				log.Fatal(err)
				return
			}
			subdir := p.Name()
			err = c.CreateSubDir(subdir)
			if err != nil {
				log.Fatal(err)
				return
			}
			err = c.WriteToSubDir(bytes, p.Name(), subdir)
			if err != nil {
				log.Fatal(err)
				return
			}
			err = c.CopyTemplate(p.Name(), subdir)
			if err != nil {
				log.Fatal(err)
				return
			}
		}(h)
	}
	wg.Wait()
	return nil
}
