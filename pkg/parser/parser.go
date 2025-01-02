package parser

import (
	"github.com/janhoon/tree-sitter-sql/bindings/go"
	"github.com/tree-sitter/go-tree-sitter"
)

type Parser struct {
	p *tree_sitter.Parser
}

func NewSQLParser() *Parser {
	p := tree_sitter.NewParser()
	p.SetLanguage(tree_sitter.NewLanguage(tree_sitter_sql.Language()))

	return &Parser{p: p}
}

func (p *Parser) Parse(input []byte) (*tree_sitter.Tree, error) {
	tree := p.p.Parse(input, nil)
	if tree == nil {
		return nil, &tree_sitter.LanguageError{}
	}

	return tree, nil
}

func (p *Parser) Close() {
	p.p.Close()
}
