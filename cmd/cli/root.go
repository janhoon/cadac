package cli

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "cadac",
	Short: "The Cadac CLI",
	Long: `
# Cadac CLI

Cadac is a data transformation and cataloging tool being develeped to allow data teams to
transform, test, catalag and share data withing their organizations.`,
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
