package util

import (
	"net/url"
	"path"
)

func Unique(target []string) (unique []string) {
	m := map[string]bool{}

	for _, v := range target {
		if !m[v] {
			m[v] = true
			unique = append(unique, v)
		}
	}

	return unique
}

func Join(endpoint, p string) string {
	u, _ := url.Parse(endpoint)
	u.Path = path.Join(u.Path, p)
	return u.String()
}
