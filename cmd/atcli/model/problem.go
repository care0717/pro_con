package model

import (
	"github.com/care0717/pro_con/cmd/atcli/util"
	"golang.org/x/net/html"
	"golang.org/x/net/html/atom"
	"io"
	"net/http"
	"path"
	"strings"
)

type Sample struct {
	Input  string `json:"input"`
	Output string `json:"output"`
}

func NewProblem(href string) Problem {
	return Problem{href: href}
}

type Problem struct {
	href string
}

func (p Problem) ContestName() string {
	return strings.Split(path.Base(p.href), "_")[0]
}

func (p Problem) Name() string {
	return strings.Split(path.Base(p.href), "_")[1]
}
func (p Problem) FetchSamples(client *http.Client) ([]Sample, error) {
	url := util.Join(endpoint, p.href)

	r, err := client.Get(url)
	if err != nil {
		return nil, err
	}
	defer r.Body.Close()
	return ParseSample(r.Body)
}

func ParseSample(body io.Reader) ([]Sample, error) {
	node, err := html.Parse(body)
	if err != nil {
		return nil, err
	}

	res := util.Unique(findSampleTexts(node))
	samples := make([]Sample, len(res)/2)
	for i := 0; i < len(res)/2; i++ {
		samples[i] = Sample{
			Input:  res[2*i],
			Output: res[2*i+1],
		}
	}
	return samples, nil
}

func findSampleTexts(node *html.Node) []string {
	if node.DataAtom == atom.Pre {
		for c := node.FirstChild; c != nil; c = c.NextSibling {
			if c.Type == html.TextNode && strings.TrimSpace(c.Data) != "" {
				return []string{strings.TrimRight(c.Data, "\n")}
			}
		}
	}
	var res []string
	for c := node.FirstChild; c != nil; c = c.NextSibling {
		if c.Type == html.ElementNode {
			if s := findSampleTexts(c); len(s) > 0 {
				res = append(res, s...)
			}
		} else {
			continue
		}

	}
	return res
}
