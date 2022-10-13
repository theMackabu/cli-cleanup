package main

import (
	"os"
	"fmt"
	"path"
	"strings"
	"path/filepath"
)

func moveFile(file1, file2 string) error {
	return os.Rename(file1, file2)
}

func main() {
	homeDir, err := os.UserHomeDir()
	 if err != nil {
		  panic(err)
	 }
	 
	desktop := path.Join(homeDir, "/Desktop/Screen*")
	files, _ := filepath.Glob(desktop)

	if (len(files) == 0) {
		fmt.Println("\033[1;33mno screenshots on the desktop\033[0m")
	} else {
		fmt.Printf("\033[1;34mmoved %d file(s) to the screenshots folder.\033[0m\n", len(files))
		for _, fileName := range files {
			pathArr := strings.Split(fileName, "/")
			screenshots := path.Join(homeDir, "/Pictures/Screenshots", pathArr[len(pathArr)-1])
			 if err := moveFile(fileName, screenshots); err != nil {
				  panic(err)
			 }
		}
	}
}