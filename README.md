# Snippets

Markus Gumbel

Version 0.1.x, December 2022

#### Abstract

Snippets is a tool that supports the presentation and description of programs in documents like presentation, books etc. It aims at teachers who present tons of source code to their audience or authors who write a text book on programming. Snippet can

 * extract snippets (fragments of any source code or text) which can be included in a text document or further processed elsewhere,
 * hide or publish solutions,
 * omit private source code or text.

Snippets follows the _single source_ principle. You do not copy and paste source code or text - instead the tool does this for you. Currently, snippets works for all programming languages which have single line comments (like // in Java, C/C++, Rust or # in Python/R). snippets can also be used for markdown languages like R markdown.

## Prerequisites and installation

Snippets comes as a single command line tool, `snips` under Linux or `snips.exe` under Windows.

## Snippets

### Definition of a snippet

In its simplest form, a snippet is an extract of a source code. Usually, the complete source code of a working example is to long to present it in a listing. Here you may want to create a snippet which highlights the statements you want to explain.

Let us consider the following complete Java program in the file `Foo.java`.

```
public class Foo {
  public static void main(String[] args) {
    // +IN Slide
    int a = 1;
    // -IN Slide
   System.out.println("Value is " + a);
  }
}
```
The statements `+IN` and `-IN` in the comments will create a snippet `Foo_Slide.java` which contains

```
...
    int a = 1;
...
```
The word `Slide` after `+IN` (and `-IN`) indicates a label for this snippet. The tool Snippets simply looks for some keywords in single line commands. You may have noticed that snippets embeds the extract with dots (…) to indicate that not the entire listing is shown.

Of course, we can add more snippets and nest them if needed.

```
public class Foo {
  // +IN Slide_main
  public static void main(String[] args) {
    // +IN Slide_statement
    int a = 1;
    // -IN Slide_statement
   System.out.println("Value is " + a);
  }
  // -IN Slide_main
}
```
This will create two snippets:

1) `Foo_Slide_statement.java`

```
...
    int a = 1;
...
```

2)  `Foo_Slide_main.java`

```
...
  public static void main(String[] args) {
    int a = 1;
    System.out.println("Value is " + a);
  }
...
```

### Permanently hide statements

Sometimes is is useful to hide statements in the source code or text in general. This can be achieved with the `+OUT` and `-OUT` keywords.

```
public class Foo {
  // +IN Slide_main
  public static void main(String[] args) {
    int a = 1;
    // +OUT
    // This will be excluded.
    System.out.println("Hello Word!");
    // -OUT
    System.out.println("Value is " + a);
  }
  // -IN Slide_main
}
```

The snippet `Foo_main.java` still contains

```
...
  public static void main(String[] args) {
    int a = 1;
    System.out.println("Value is " + a);
  }
...
```

Snippet will create the snippets in a specified folder (e. g. `snips`). Also, snippet will _copy_ all scanned source files to an additional specified folder (e. g. `src_dest`). Copy means that the source files will not contain any statements which were excluded with the `OUT` option. This is useful if you want to publish your source code but want to exclude several statements. 

### Publishing sample solutions

In a teaching scenario you may want to give your students exercises which contain some source code. However, the solution of the exercise must not be published before the exercises are finished. There is a snippet option `EXC` (like exercise) which prevents the publication of the solution. `EXC` is similar to `OUT` with the difference that snippet also copies the scanned source files to an additional specified folder (e. g. `src_dest_solution`) that contains the solutions.

As an example let us assume we ask the students to output the value of the var `a` to the console. The following source code will do this:

```
public class Foo {
  public static void main(String[] args) {
    int a = 1;
    // +OUT
    // This will be excluded.
    System.out.println("Hello Word!");
    // -OUT
    // +EXC
    // This is the solution:
    System.out.println("Value is " + a);
    // -EXC
  }
}
```

The public folder (e.g. `src_dest`) will contain a file `Foo.java` with the content

```
public class Foo {
  public static void main(String[] args) {
    int a = 1;
  }
}
```
The sample solution folder (e.g. `src_dest_solution`) will also contain a file `Foo.java` but with the content

```
public class Foo {
  public static void main(String[] args) {
    int a = 1;
    // This is the solution:
    System.out.println("Value is " + a);
  }
}
```

Note that the excluded statements (option `OUT`) are always missing.

There are situations where the sample solution cannot simply be omitted because then the program would not compile any more. Consider the exercise where we are asked to override the `toString()` method of class `Bar`. This method has a return value and the Java compiler expects to see such a return statement. This solution would not compile:

```
public class Bar {
  public String toString() {
    // Please return "Hello from Bar!"
    // +EXC
    // This is the solution:
    return "Hello from Bar!";
    // -EXC
  }
}
```

The reason is that the public version in `src_dest` looks like this. 

```
public class Bar {
  public String toString() {
     // Please return "Hello from Bar!"
  }
}
```
The return statement is missing!

We can use a variant of option `EXC` instead. `EXCSUBST` substitutes the hidden block with a single statement.

```
public class Bar {
  public String toString() {
    // Please return "Hello from Bar!"
    // +EXCSUBST 4 return null; // Not yet completed.
    // This is the solution:
    return "Hello from Bar!";
    // -EXCSUBST
  }
}
```
The public version (in `src_dest`) will then contain

```
public class Bar {
  public String toString() {
    // Please return "Hello from Bar!"
    return null; // Not yet completed.
  }
}
```
which compiles fine. `EXCSUBST` reads the first white space (here `4`) and interprets this as the number of spaces which it will indent the following statement (`return null`). The solution (in `src_dest_solution`) looks like:

```
public class Bar {
  public String toString() {
    // Please return "Hello from Bar!"
    // This is the solution:
    return "Hello from Bar!";
  }
}
```

###  Markdown documents

This concept can also be applied for text documents which are based on
pure text like markdown or LaTeX. We will show an example for R markdown. The following documents contains some text and a code chunk in R (the left directed ' are displayed as normal ' for simplicity).

```
---
title: "Snips R markdown file"
output: html_notebook
---

# header 1

// +EXC

This is the solution. Please note that a # cannot be used as a comment in R markdown as it indicates a header. So // is used instead.

// -EXC

# header 2

This is an example with a code chunk:
'''{r}
a = 1
# +EXC
# This code was removed in the public solution:
print(a)
# -EXC
'''
```

Note that there were two escape comments used. 1) // for markdown and 2
) # for R comments. This is useful because a # in markdown would render a header. Thus, any snippet option would be displayed as a header in the primary document. Telling snippet to use // as a comment with snippet options mitigates this.

The public version (in `src_dest`) will then contain

```
---
title: "Snips R markdown file"
output: html_notebook
---

# header 1


# header 2

This is an example with a code chunk:
'''{r}
a = 1
'''
```

This hides some text and some programming statements. The solution (in `src_dest_solution`) looks like:

```
---
title: "Snips R markdown file"
output: html_notebook
---

# header 1

This is the solution. Please note that a # cannot be used as a comment in R markdown as it indicates a header. So // is used instead.

# header 2

This is an example with a code chunk:
'''{r}
a = 1
# This code was removed in the public solution:
print(a)
'''
```

### Execution

Let us assume the following scenario: The current working directory (`./`) contains several sub-directories. We have Java source files in the folder `./src` with the packages `a` and `b` as folders. Now we want to store any extracted files in folder `./variants`. `snips` therein will contain all _public_ snippets, i.e snippets without any solutions and `snips_solution` will contain the same snippets but with the solution included (those embedded in the `EXC` or `EXCSUBST` flags). The same happens with the source files in `src_dest` or `src_dest_solution`, respectively. Note that the package structure is copied, i.e. the folder `a` and `b` also exist in the directories `src_dest` and `src_dest_solution`.

```
| src                     # Java source code
  | a                     # Java package a
  | b                     # Java package b
| variants                # Root folder for generated files
   | snips                # snips
   | snips_solution       # snips with solutions
   | src_dest             # Public Java source files
      | a                 #  including packages 
      | b                 #  w/o OUT sections
   | src_dest_solution    # Java source files with solutions
      | a                 #  including packages
      | b                 #  w/o OUT sections but with EXC
```

#### Publish public files

The following example scans all Java files (`*.java`) in the directory `src` and writes the snippets to `snips`. The single line comment is `//` as Java files are scanned.

`snips -s ./src -t ./variants/snips -d ./variants/src_dest -c "//" -x *.java`

The published source code files are written to `./variants/snips`. `snips` does not add the solutions as the parameter `-e` (or `--exercise_solution`) is not set. I. e. any statements within `EXC` or `EXCSUBST` are omitted or substituted.

#### Publish solution files

We now want to publish the solutions. The only differences to the example above are that we 1) set the `--exercise_solution` flag and store the output in alternative directories (with suffix `_solution`).

`snips -s ./src -t ./variants/snips_solution -d ./variants/src_dest_solution -c "//" -x *.java -e`

### Command line syntax

The usage of the command line tool is:

```
snips [OPTIONS] -s <src_dir>

OPTIONS:
    -a <alternative_comment>        Alternative escape comment symbol [default: //]
    -c <comment>                    Escape comment symbol, e.g. # or // [default: #]
    -d <src_dest_dir>               Directory where stripped source files will be stored. [default:
                                    ./src_dest]
    -e <exercise_solution>          Include solutions (EXC and EXCSUBST flags) [default: false]
    -h, --help                      Print help information
    -s <src_dir>                    Directory with source files.
    -t <snippet_dest_dir>           Directory where snippet files will be stored. [default:
                                    ./snippets]
    -v                              Add this flag multiple times to increase message verbosity
    -V, --version                   Print version information
    -x <file_suffix>                File suffix of files to process. [default: .txt]
```    
