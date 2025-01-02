package parser

import (
	"testing"
)

func TestSQLParser(t *testing.T) {
	tests := []struct {
		name    string
		input   string
		wantErr bool
	}{
		{
			name:    "simple select statement",
			input:   "SELECT * FROM users",
			wantErr: false,
		},
		{
			name:    "invalid sql statement",
			input:   "SELEC * FROM users",
			wantErr: true,
		},
		{
			name:    "complex select statement",
			input:   "SELECT id, name FROM users WHERE age > 18 ORDER BY name DESC",
			wantErr: false,
		},
		{
			name:    "empty input",
			input:   "",
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			p := NewSQLParser()
			defer p.Close()

			tree, err := p.Parse([]byte(tt.input))

			if (err != nil) != tt.wantErr {
				t.Errorf("Parse() error = %v, wantErr %v", err, tt.wantErr)
				return
			}

			if !tt.wantErr && tree == nil {
				t.Error("Parse() returned nil tree for valid input")
			}
		})
	}
}
