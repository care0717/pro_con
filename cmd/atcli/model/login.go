package model

import (
	"errors"
	"github.com/care0717/pro_con/cmd/atcli/util"
	"golang.org/x/net/html"
	"golang.org/x/net/html/atom"
	"net/http"
	"net/url"
	"os"
)

func Login(client *http.Client, username, password string) error {
	username = os.Getenv("ATCODER_USERNAME")
	password = os.Getenv("ATCODER_PASSWORD")
	loginUrl := util.Join(endpoint, "login")
	r, err := client.Get(loginUrl)
	if err != nil {
		return err
	}
	node, err := html.Parse(r.Body)
	if err != nil {
		return err
	}
	defer r.Body.Close()
	token, err := findCSRF(node)
	if err != nil {
		return err
	}
	res, err := client.PostForm(loginUrl, url.Values{
		"username":   {username},
		"password":   {password},
		"csrf_token": {token},
	})
	if err != nil {
		return err
	}
	defer res.Body.Close()
	return nil
}

func findCSRF(node *html.Node) (string, error) {
	if node.DataAtom == atom.Input {
		for _, v := range node.Attr {
			if v.Key == "value" {
				return v.Val, nil
			}
		}
	}
	for c := node.FirstChild; c != nil; c = c.NextSibling {
		if c.Type == html.ElementNode {
			if s, _ := findCSRF(c); s != "" {
				return s, nil
			}
		} else {
			continue
		}

	}
	return "", errors.New("cannot find csrf")
}
