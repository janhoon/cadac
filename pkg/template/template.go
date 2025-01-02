package template

import (
	"io"
	"text/template"
)

type Template struct {
	t template.Template
}

func NewTemplate() *Template {
	return &Template{}
}

func (t *Template) Execute(wr io.Writer, data interface{}) error {
	return t.t.Execute(wr, data)
}
