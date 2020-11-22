package model

import (
	"fmt"
	"github.com/care0717/pro_con/cmd/atcli/util"
	"golang.org/x/net/html"
	"golang.org/x/net/html/atom"
	"io"
	"io/ioutil"
	"net/http"
	"os"
	"path"
	"regexp"
	"strings"
)

func NewContest(name string) Contest {
	return Contest{name: name}
}

type Contest struct {
	name string
}

func (c Contest) ValidContest(client *http.Client) bool {
	r, err := client.Get(c.Url())
	if err != nil {
		return false
	}
	return r.StatusCode == http.StatusOK
}

func (c Contest) Url() string {
	p := fmt.Sprintf("/contests/%s/tasks", c.name)
	return util.Join(endpoint, p)
}

func (c Contest) ProblemPrefix() string {
	return fmt.Sprintf("/contests/%s/tasks/%s_", c.name, c.name)
}

func (c Contest) FetchProblemHrefs(client *http.Client) ([]string, error) {
	r, err := client.Get(c.Url())
	if err != nil {
		return nil, err
	}
	defer r.Body.Close()
	node, err := html.Parse(r.Body)
	if err != nil {
		return nil, err
	}
	urls := util.Unique(findProblemHrefs(node, c.ProblemPrefix()))
	return urls, nil
}

func findProblemHrefs(node *html.Node, prefix string) []string {
	if node.DataAtom == atom.A {
		for _, v := range node.Attr {
			if v.Key == "href" && strings.Contains(v.Val, prefix) {
				return []string{v.Val}
			}
		}
	}
	var res []string
	for c := node.FirstChild; c != nil; c = c.NextSibling {
		if c.Type == html.ElementNode {
			if s := findProblemHrefs(c, prefix); len(s) > 0 {
				res = append(res, s...)
			}
		} else {
			continue
		}

	}
	return res
}

var alphabet = regexp.MustCompile(`[a-zA-z]+`)

func (c Contest) WriteToSubDir(b []byte, problemName, subdir string) error {
	err := ioutil.WriteFile(path.Join(c.DirName(), subdir, fmt.Sprintf("%s.json", problemName)), b, 0644)
	if err != nil {
		return err
	}
	return nil
}

func (c Contest) DirName() string {
	alphabet.Longest()
	return path.Join("atcoder", alphabet.FindString(c.name), alphabet.ReplaceAllString(c.name, ""))
}

func (c Contest) CreateDir() error {
	return os.MkdirAll(c.DirName(), 0755)
}

func (c Contest) ExistDir() bool {
	_, err := os.Stat(c.DirName())
	return !os.IsNotExist(err)
}

func (c Contest) CreateSubDir(subdir string) error {
	return os.MkdirAll(path.Join(c.DirName(), subdir), 0755)
}

func (c Contest) CopyTemplate(name, subdir string) error {
	src, err := os.Open("library/template/template.go")
	if err != nil {
		return err
	}
	defer src.Close()

	dst, err := os.Create(path.Join(c.DirName(), subdir, fmt.Sprintf("%s.go", name)))
	if err != nil {
		return err
	}
	defer dst.Close()

	_, err = io.Copy(dst, src)
	if err != nil {
		return err
	}
	return nil
}
