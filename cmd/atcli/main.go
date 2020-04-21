package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"github.com/care0717/pro_con/cmd/atcli/model"
	"io"
	"io/ioutil"
	"log"
	"os"
	"os/exec"
	"path"
	"strings"
	"sync"

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
	for i, s := range samples {
		cmd := exec.Command("go", "run", path.Join(c.DirName(), subdir, fmt.Sprintf("%s.go", problemName)))
		stdin, _ := cmd.StdinPipe()
		if _, err := io.WriteString(stdin, s.Input); err != nil {
			return nil
		}
		stdin.Close()
		out, _ := cmd.Output()
		actual := strings.TrimRight(string(out), "\n")
		if actual == s.Output {
			fmt.Printf("case %d OK\n", i)
		} else {
			fmt.Printf("expect %s, but got %s\n", s.Output, actual)
		}
	}
	return nil
}

func create(contestName string) error {
	c := model.NewContest(contestName)

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
