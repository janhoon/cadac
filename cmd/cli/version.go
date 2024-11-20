package cli

import (
	"fmt"

	"github.com/spf13/cobra"
)

func init() {
	rootCmd.AddCommand(versionCmd)
}

var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "Print the version number of Cadac CLI",
	Long:  `All software has versions. This is Cadac's`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Printf("Cadac CLI version %s\n", getVersion())
	},
}

func getVersion() string {
	// TODO: get version from somewhere that makes more sense
	return "0.0.1"
}
