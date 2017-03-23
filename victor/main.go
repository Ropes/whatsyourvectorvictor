/*victor returns the NATO alphabet designations responding to the
given character or numeric index.

eg:
	$ victor 1
	alpha

	$ victor b
	bravo
*/
package main

import (
	"flag"
	"fmt"
	"strconv"
	"unicode"
	"unicode/utf8"

	wyvv "github.com/ropes/whatsyourvectorvictor"
)

func main() {
	flag.Parse()
	args := flag.Args()
	if len(args) < 1 {
		fmt.Printf("A vector must be specified victor! %v\neg: victor 13\n", args)
	}

	s := args[0]
	r, _ := utf8.DecodeRuneInString(s)
	b := unicode.IsDigit(r)

	if !b {
		//fmt.Printf("Unable to convert arg['%s'] to number: %v", args[0], err)
		vector := wyvv.TranslateRune(r)
		fmt.Printf("%s\n", vector)
	} else {
		num, _ := strconv.Atoi(s)
		num = num - 1
		if num >= 0 && num < 26 {
			fmt.Printf("%s\n", string(wyvv.Alphabet[num]))
		} else {
			fmt.Printf("number specified must be between 1-26: received %s\n", s)
		}
	}
}
