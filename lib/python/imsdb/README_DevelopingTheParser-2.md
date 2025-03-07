
# Commands to use

Listing the commands at the top for reference
 - In vscode terminal or elsewhere. To generate the python parser from the grammar file
   - `antlr4 -Dlanguage=Python3 -o ../antlrgen Screenplay.g4`
   - Note that `antlr4` is an alias: `alias antlr4='java -Xmx500M org.antlr.v4.Tool'`
 - To run the generated parser
   - `cd ~/bitbucket/hillops/libs/python/hillops/imsdb`
   - `python screenplay_parser.py` _to run with hardcoded samples in the code_
   - `python screenplay_parser.py samples/aladdin.txt > log.txt` _to run against the aladdin script sample or any other script from imsdb. Only errors show up console while parsed json output ends up in the log.txt file_.

# Redo grammar 

Doing some study of Antlr4 _(while I await the book)_, I need to work around old thought of predicates. Antlr4 has apprently fixed the parse weakness of Antlr3 and one no longer needs to use syntactic predicates _(purely lexer, token sequence based predicate logic where you might think of look-behind or look-ahead assertions described in terms of tokens)_.

Easier to start fresh rather than edit the current grammar. Starting with the simplified _(removed the `UPPER` and `LOWER` fragments) lexer rules:


## Original grammar

```antlr
// Great resource - https://tomassetti.me/antlr-mega-tutorial/
// Collection of Antlr4 grammars: https://github.com/antlr/grammars-v4
grammar Screenplay;

// parser rules
screenplay          : (
                        actor_section 
                        | 
                        scene_section
                      )+ EOF
                    ;

// Actor sections can have scene lines as well.
// that belong to the actor. Should be in the same para
// without an empty line between.
actor_section       : actor_name 
                        (
                            section_line
                            |
                            scene_section
                        ) +
                        (
                            CR 
                            | 
                            EMPTY_LINE
                            |
                            EOF
                        ) ?
                    ;

actor_name          : WORD COLON
                    ;                    

section_line        : WS? (WORD WS? PUNCT? WS?)+
                    ;

scene_section       : WS? PARENS_OPEN 
                        (
                            section_line
                            |
                            scene_section
                        )+ 
                        PARENS_CLOSE WS? CR?
                    ;

// Lexer rule names begin with upper-case letters
// Really simple stuff
// Simplify WORD to not include a ':' or '(' both of which 
// are semantically meaningful in this format.
WORD               : ~[ \n\r\t:()]+ (WS | CR | EOF)?
                   ;

PUNCT              : [.,?!] 
                   ;

COLON              : ':'
                   ;

PARENS_OPEN        : '('
                   ;

PARENS_CLOSE       : ')'
                   ;                   

fragment LOWER     : [a-z]
                   ;

fragment UPPER     : [A-Z]
                   ;


// Empty line is syntactically relevant so not skipping it.
// All can be clearned up after parse.
CR                 : [\r\n]+
                   ;

WS                 : [ \t]+
                   ;

EMPTY_LINE         : (CR WS* CR)+
                   ;                   
```


```antlr
// Lexer rule names begin with upper-case letters
// Really simple stuff
// Simplify WORD to not include a ':' or '(' both of which 
// are semantically meaningful in this format.
WORD               : ~[ \n\r\t:()]+
                   ;

PUNCT              : [.,?!] 
                   ;

COLON              : ':'
                   ;

PARENS_OPEN        : '('
                   ;

PARENS_CLOSE       : ')'
                   ;

// Empty line is syntactically relevant so not skipping it.
// All can be clearned up after parse.
CR                 : [\r\n]+
                   ;

WS                 : [ \t]+ -> skip
                   ;

EMPTY_LINE         : (CR WS* CR)+
                   ;                   
```

How can I build the rules up better ? `actor_section` and `scene_section` seem reasonable.
 - `actor_section`
   - `WORD` not including `:` is still suspect!
   - starts on column 0
   - actor_name followed by text on many lines
   - ends when 
     - Next line is empty
     - When not-empty
       - next line is another `actor_section` which starts with `actor_name` ()
       - next line is a scene section that starts with `(` _again seems to be column 0_
       - OTW part of the current `actor_section`
     - How to convert the above assertions to plain parser rules without semantic assertions ?
       - Lot more tokens when possible ?
       - Convert `actor_name` into `ACTOR_NAME` token. This will match more characters than any of the other tokens so will be selected first.

## Just actor lines, no parenthenrical text

```python
sample = """ALADDIN:    Sultan?  They want me to be sultan?

GENIE:  Huzzah!  Hail the conquering hero!  
        Aladdin, you've just won the heart of the princess.  
        What are you gonna do next?
"""
```

Remove other rules and examine the grammar.

```antlr
// parser rules
screenplay          : actor_section*
                      EOF
                    ;

actor_section       : actor_name section_line+
                    ;

actor_name          : ACTOR_NAME
                    ;                    

section_line        : (WORD PUNCT?)+ CR?
                    ;

ACTOR_NAME         : {self.column == 0}? WORD COLON
                   ;

WORD               : ~[ \n\r\t:()]+
                   ;

PUNCT              : [.,?!] 
                   ;

COLON              : ':'
                   ;

PARENS_OPEN        : '('
                   ;

PARENS_CLOSE       : ')'
                   ;

CR                 : [\r\n]+
                   ;

WS                 : [ \t]+ -> skip
                   ;

EMPTY_LINE         : (CR WS* CR)+
                   ;                   

```

 - Kept `actor_name` as a rule just to keep the current listener usable since it was saving the actor name in the `enterActor_name`

 - first token will be `ACTOR_NAME`
 - then `section_line` will consume everything till and including `CR`
 - second line will generate `EMPTY_LINE` which will be skipped cuz no rule wants it
 - third-line will generate `ACTOR_NAME` again and match the `actor_section` 

This gave me the following (sight modifications in json reader to read the `ACTOR_NAME` token which now includes the colon instead of the previous `WORD`) 

```console
(screenplay (actor_section (actor_name ALADDIN:) (section_line Sultan? They want me to be sultan? \n\n)) (actor_section (actor_name GENIE:) (section_line Huzzah! Hail the conquering hero! \n) (section_line Aladdin, you've just won the heart of the princess. \n) (section_line What are you gonna do next? \n)) <EOF>)
{
    "sections": [
        {
            "name": "ALADDIN:",
            "content": [
                "Sultan? They want me to be sultan?"
            ]
        },
        {
            "name": "GENIE:",
            "content": [
                "Huzzah! Hail the conquering hero!",
                "Aladdin, you've just won the heart of the princess.",
                "What are you gonna do next?"
            ]
        }
    ]
}
```

👉 first section line ends with `\n\n`. Can I use the EMPTY_LINE token to split things out ? If I need to replace the predicate rules with parse rules. How about this change. If this works, 
 - ✔️ I should not see `CR` or `EOF` as part of `section_line`: However, two `CR` tokens show up after the `section_line`. Would all non-skipped tokens till the next parser_rule show up?
 - EMPTY_LINE token should break the `actor_section` but that is not used anywhere in the rules. Maybe I can skip it then ?

```diff
-actor_section       : actor_name section_line+
actor_section        : actor_name 
+                        (
+                            section_line CR?
+                        )+                    
                    ;

-section_line        : (WORD PUNCT?)+ CR??
+section_line        : (WORD PUNCT?)+
                     ;
``` 

## Add scene section lines

```python
sample_actors_plus_scene = """ALADDIN:    Sultan?  They want me to be sultan?

(GENIE comes out of lamp)

GENIE:  Huzzah!  Hail the conquering hero! Aladdin, you've just won
        the heart of the princess.  What are you gonna do
        next?
        Psst, your line is "I'm going to free the genie."
        Anytime.
"""
```

Scene section is composed of 
 - `(` starting on the first column.
 - read-everything else taking care of nested `(` and `)` _add this feature next_
 - stop at `)`
 - expect optional CR _(since WS is skipped)

Originally i thought the lexer predicate of `self.column == 0` can be used in the parser as well. But alas no. So adding a special token for `(` if it appears at the start of a line.

```diff
-screenplay          : actor_section*
+screenplay          : (
+                        actor_section
+                        |
+                        scene_section
+                      )*
+                      EOF
+                    ;
+
+scene_section       : PARENS_OPEN_COL0 
+                        (
+                            section_line CR?
+                        )+ 
+                        PARENS_CLOSE
+                    ;
+PARENS_OPEN_COL0   : {self.column == 0}? '('
+                   ;

-PARENS_OPEN        : '('
+PARENS_OPEN        : {self.column == 0}? '('
                   ;
```  

With this, things worked but there was a warning about the empty lines after the end of the scene. It said `line 3:25 extraneous input '\n\n' expecting {<EOF>, ACTOR_NAME, PARENS_OPEN_COL0}` so this was likely after `scene_section` ended and the `screenplay` rules alts expect any of the tokens the complaint it talking about.

Allow handling of CRs via

```diff
screenplay          : (
-                        actor_section
+                        actor_section (CR | EMPTY_LINE)*
                        |
-                        scene_section
+                        scene_section (CR | EMPTY_LINE)*
                      )*
                      EOF
                    ;
```

Now I get

```console
(screenplay (actor_section (actor_name ALADDIN:) (section_line Sultan? They want me to be sultan?) \n\n) (scene_section ( (section_line GENIE comes out of lamp) )) \n\n (actor_section (actor_name GENIE:) (section_line Huzzah! Hail the conquering hero! Aladdin, you've just won) \n (section_line the heart of the princess. What are you gonna do) \n (section_line next?) \n (section_line Psst, your line is "I'm going to free the genie.") \n (section_line Anytime.) \n) <EOF>)
{
    "sections": [
        {
            "name": "ALADDIN:",
            "content": [
                "Sultan? They want me to be sultan?"
            ]
        },
        {
            "content": "GENIE comes out of lamp"
        },
        {
            "name": "GENIE:",
            "content": [
                "Huzzah! Hail the conquering hero! Aladdin, you've just won",
                "the heart of the princess. What are you gonna do",
                "next?",
                "Psst, your line is \"I'm going to free the genie.\"",
                "Anytime."
            ]
        }
    ]
}
```

## Allow actor scene lines

Actor scene lines are parenthetical lines inside an actor's section.

```python
"""ALADDIN:    Sultan?  They want me to be sultan?

(GENIE comes out of lamp)

GENIE:  Huzzah!  Hail the conquering hero!  (Turns into a
        one-man band.  He sees ALADDIN walk away with his
        head hung.  He stops, scratches his head, comes up
        with an idea, then zooms over to ALADDIN.  He holds
        up his hands like a director scoping a picture and
        we look through them.)  Aladdin, you've just won
        the heart of the princess.  What are you gonna do
        next?  (ALADDIN looks at him, then walks away in
        sadness to the bed, where he falls on it and sighs.
        GENIE again is confused, then goes to him and pulls
        out a script labeled "Aladdin." Whispering)
        Psst, your line is "I'm going to free the genie."
        Anytime.
"""
```

The previously used logic of embedding a scene_section will work. However, since scene_section was changed to start with `PARENS_OPEN_COL0`, I'll add a new rule called `actor_scene_section` and then adjust `actor_section` to be composed of either: retain the terminal `CR?`. Also update the listener classes to account for this new rule.

```diff
actor_section        : actor_name 
                        (
-                            section_line CR?                                
+                            (
+                                section_line
+                                |
+                                actor_scene_section                               
+                            )
+                            CR ?
                        )+
                    ;

+actor_scene_section : PARENS_OPEN
+                        (
+                            section_line CR?
+                        )+ 
+                      PARENS_CLOSE
+                    ;
```

Bit of tweaking of the json classes and I got this to work great. A bit verbose with the `type` field and the `SceneLines` indirection but all can be cleaned for final output.

```json
{
    "sections": [
        {
            "name": "ALADDIN:",
            "content": [
                {
                    "content": [
                        "Sultan? They want me to be sultan?"
                    ],
                    "type": "SectionLines"
                }
            ],
            "type": "ActorSection"
        },
        {
            "content": {
                "content": [
                    "GENIE comes out of lamp"
                ],
                "type": "SectionLines"
            },
            "type": "SceneSection"
        },
        {
            "name": "GENIE:",
            "content": [
                {
                    "content": [
                        "Huzzah! Hail the conquering hero!"
                    ],
                    "type": "SectionLines"
                },
                {
                    "content": {
                        "content": [
                            "Turns into a",
                            "one-man band. He sees ALADDIN walk away with his",
                            "head hung. He stops, scratches his head, comes up",
                            "with an idea, then zooms over to ALADDIN. He holds",
                            "up his hands like a director scoping a picture and",
                            "we look through them."
                        ],
                        "type": "SectionLines"
                    },
                    "type": "ActorSceneSection"
                },
                {
                    "content": [
                        "Aladdin, you've just won",
                        "the heart of the princess. What are you gonna do",
                        "next?"
                    ],
                    "type": "SectionLines"
                },
                {
                    "content": {
                        "content": [
                            "ALADDIN looks at him, then walks away in",
                            "sadness to the bed, where he falls on it and sighs.",
                            "GENIE again is confused, then goes to him and pulls",
                            "out a script labeled \"Aladdin.\" Whispering"
                        ],
                        "type": "SectionLines"
                    },
                    "type": "ActorSceneSection"
                },
                {
                    "content": [
                        "Psst, your line is \"I'm going to free the genie.\"",
                        "Anytime."
                    ],
                    "type": "SectionLines"
                }
            ],
            "type": "ActorSection"
        }
    ],
    "type": "ScreenPlay"
}
```

## Allow colon terminated words inside non-actor contexts

 Since `ACTOR_NAME` is now predicated on `{self.column == 0}`. 

 With the following sample (Note the **Whispeing:** close to the end of **GENIE:**)

```python
 samples["sample_actors_plus_scene_plus_actor_scene_plus_colonwords"] = """ALADDIN:    Sultan?  They want me to be sultan?

(GENIE comes out of lamp)

GENIE:  Huzzah!  Hail the conquering hero!  (Turns into a
        one-man band.  He sees ALADDIN walk away with his
        head hung.  He stops, scratches his head, comes up
        with an idea, then zooms over to ALADDIN.  He holds
        up his hands like a director scoping a picture and
        we look through them.)  Aladdin, you've just won
        the heart of the princess.  What are you gonna do
        next?  (ALADDIN looks at him, then walks away in
        sadness to the bed, where he falls on it and sighs.
        GENIE again is confused, then goes to him and pulls
        out a script labeled "Aladdin." Whispering:)
        Psst, your line is "I'm going to free the genie."
        Anytime.
"""
```

With no changes, I get `line 15:50 extraneous input ':' expecting {WORD, ')', CR}`. The problem is that the token `WORD` disallows `:`. Not sure if this should be at the token level or at the parse level.

If token level, I can imagine this now.

```diff
-ACTOR_NAME         : {self.column == 0}? WORD COLON
+ACTOR_NAME         : WORD_COL0 COLON
                   ;

-WORD               : ~[ \n\r\t:()]+
-                   ;

+// Does not allow ':'
+WORD_COL0          : ~[ \n\r\t:()]+ 
+                   ;
+
+// Allows ':'
+WORD               : ~[ \n\r\t()]+
+                   ;                                      

section_line        : (WORD PUNCT?)+
                    ;
```

🎉 that worked!

```json
 {
                    "content": {
                        "content": [
                            "ALADDIN looks at him, then walks away in",
                            "sadness to the bed, where he falls on it and sighs.",
                            "GENIE again is confused, then goes to him and pulls",
                            "out a script labeled \"Aladdin.\" Whispering:"
                        ],
                        "type": "SectionLines"
                    },
                    "type": "ActorSceneSection"
                },
```

## Allow nested parens inside existing parens

> Note: This fix has limitations. 
> - It does not count '(' ')' that can be inside double quotes. 
> - Also does not limit the damage from unclosed parens (by always breaking on empty lines etc)

Ok. This might be a bit more complex. After some tinkering and experienting with counting parens though, was able to end up with a simple set of changes.

```diff
section_line_parens : WORD+ (PARENS_OPEN ~PARENS_CLOSE* PARENS_CLOSE)?
                    ;                    

scene_section       : PARENS_OPEN_COL0
                        (
+                          section_line_parens CR?
-                          section_line CR?
                        )+ 
                      PARENS_CLOSE
                    ;

actor_scene_section : PARENS_OPEN
                        (                                                    
+                          section_line_parens CR?
-                          section_line CR?
                        )+ 
                      PARENS_CLOSE
                    ;
```

Now, a different approach would be to use semantic predicates. That would look like the following. I dropped that idea as the grammar looked more complex and wasn't sure I needed it any more.

```diff
+// Needed some trial and error to match the indent in the 
+// __init__ method of the parser where this ends up.
+ @parser::members {    self.open_p = 0}

-section_line        : WORD+ 
+section_line   : {self.open_p == 0}? WORD+ 
+               | {self.open_op > 0}? WORD+ (PARENS_OPEN ~PARENS_CLOSE* PARENS_CLOSE)?
                ;

scene_section   : PARENS_OPEN_COL0                {self.open_p = self.open_p + 1}
                    (
                      section_line_parens CR?
                    )+ 
                  PARENS_CLOSE                    {self.open_p = self.open_p - 1}
                ;

actor_scene_section : PARENS_OPEN                  {self.open_p = self.open_p + 1}
                        (                                                    
                          section_line_parens CR?
                        )+ 
                      PARENS_CLOSE                  {self.open_p = self.open_p - 1}
                    ;                
```

## Problem - Allow names like `GUARD 1:`

This problem took me some trials to get right. I thought this was simple at first and tried the following change

```diff
-ACTOR_NAME         : WORD_COL0 COLON
+ACTOR_NAME         : WORD_COL0 DIGIT*? COLON
                   ;

+DIGIT              : [0-9]
+                   ;
```

and I still get a `line 18:0 mismatched input 'GUARD' expecting {<EOF>, ACTOR_NAME, PARENS_OPEN_COL0, CR, EMPTY_LINE}`

Why is ACTOR_NAME not picking it up ? I thought the rule was 
 - If more than one lexer rule match the input (`ACTOR_NAME`, `WORLD_COL0` )
   - The rule that matches a longer set of characters is used
   - OTW, the rule listed earlier in the grammar is used

From the error message.
 - The next token being expected by the rules 
   - `actor_section` expected `ACTOR_NAME`
   - `screenplay` expects 
     - `actor_section` or CR or EMPTY_LINE or EOF
     - `scene_section` or CR or EMPTY_LINE or EOF
 - These together expect the first token in their rule which ends up with `EOF`, `ACTOR_NAME`, `PARENS_OPEN_COL0`, `CR` and `EMPTY_LINE`

So.. What token is being generated here then if not `ACTOR_NAME` ?
 - Tried to see if I could use the graphical debugger (`grun`) but that was only for the java parser.
 - Spent a few minutes staring at it _(could have simply printed the tokens to see what was happening honsetly)_ when i finally got it. Mutiple mistakes!!

  - `DIGIT` was never being hit because `WORD` includes everyhing! Doh!! Likely I was getting a `[WORD_COL0 WORD COLON]` sequence.
  - One way to deal with this might have been to list `DIGIT` before `WORD`
  - `WS ->skip` means WS, if matched will be skipped as input to the parser. However, `WORD_COL0 DIGIT` will never match if these is a space between. An assumed `Ws ->skip` will never occur between them. So the lexer rule needed `WORD_COL0 ' '*? DIGIT*? COLON`

Anyway, finally ended up with

```diff
+fragment DIGIT     : [0-9]

-ACTOR_NAME         : WORD_COL0 COLON
+ACTOR_NAME         : WORD_COL0 ' '*? DIGIT*? COLON
                    ;                   
```

## Problem - Song segments. Looks like scene section but without parens

```text
ALADDIN:    Come on, let's get outta here!
        Gotta keep...one jump ahead of the breadline
        One swing ahead of the sword
        I steal only what I can't afford
        That's everything!

(ALADDIN battles a GUARD wielding a sword.  He dodges a couple of
    swings, then pulls down the GUARD's pants.  ABU raspberries the
    GUARD, then dodges an attack.  The GUARD swings at ALADDIN,
    but destroys a barrel of fish.  As ALADDIN runs off, the GUARD
     pulls a fish over his lower body as a pair of pants.)

        One jump ahead of the lawmen
        That's all, and that's no joke
        These guys don't appreciate I'm broke!

```

For now, don't bother marking them as such. Who knows what other random bits I'll find in other scripts. 

```diff
scene_section : 
+                (
                  PARENS_OPEN_COL0
                    (
                      section_line_parens CR?
                    )+ 
                  PARENS_CLOSE
+                )
+                |
+                // There are song sequences in the middle which are
+                // newline separated and indented. So WS can be 
+                // treated as significant ?
+                //   Does indent means the actor who'se lines precede it 
+                //   Sings those ? For now treat it as regular scene_section
+                (
+                    (
+                      section_line_parens CR?
+                    )+ 
+                )
              ;
```

## Problem - Refine actor name to include groups and spaces

There are sections like these, where the `role` has multiple words
 - `ALADDIN and JASMINE`
 - `OLD MAN`
 - `DUP. GENIES`
 - `CHORUS OF MEN`
 - `CHORUS OF WOMEN`
 - `MEN'S CHORUS`

```text
OLD MAN:    There is a cave, boy.  A cave of wonders.  Filled
        with treasures beyond your wildest dreams.
        Treasure enough to impress even your princess, I'd
        wager.
```

```diff
+fragment WORD_NC   : ~[ \n\r\t():]+
+                   ;

-ACTOR_NAME         : WORD_COL0 SPACE*? DIGIT*? COLON
+ACTOR_NAME         : WORD_COL0 (SPACE+? WORD_NC)* SPACE*? COLON
```

Works!

## Addtional details

There are some other characters listed in the script I think. Any all-caps name/role. Am not recording those for now. 

## And finally all done

This is what the final grammar file looks like

```antlr
// Great resource - https://tomassetti.me/antlr-mega-tutorial/
// Collection of Antlr4 grammars: https://github.com/antlr/grammars-v4
grammar Screenplay;

// parser rules
screenplay          : (
                        actor_section (CR | EMPTY_LINE)*
                        |
                        scene_section (CR | EMPTY_LINE)*
                      )*
                      EOF
                    ;

// Actor sections can have scene lines as well.
// that belong to the actor. Should be in the same para
// without an empty line between.
actor_section        : actor_name 
                        (
                            (
                                section_line
                                |
                                actor_scene_section                               
                            )
                            CR ?
                        )+
                    ;

actor_name          : ACTOR_NAME
                    ;

section_line        : WORD+ 
                    ;

section_line_parens : (
                        WORD+ 
                        |
                        PARENS_OPEN ~PARENS_CLOSE* PARENS_CLOSE?
                      )+
                    ;                    

scene_section       : (
                        PARENS_OPEN_COL0
                          (
                            section_line_parens CR?
                          )+ 
                        PARENS_CLOSE
                      )
                      |
                      // There are song sequences in the middle which are
                      // newline separated and indented. So WS can be 
                      // treated as significant ?
                      //   Does indent means the actor who'se lines precede it 
                      //   Sings those ? For now treat it as regular scene_section
                      (
                          (
                            section_line_parens CR?
                          )+ 
                      )
                    ;

actor_scene_section : PARENS_OPEN
                        (                                                    
                          section_line_parens CR?
                        )+ 
                      PARENS_CLOSE
                    ;


// Lexer rule names begin with upper-case letters
// Does not allow ':'
// Allows names like 'GUARD 33:'
fragment DIGIT     : [0-9]
                   ;

fragment SPACE     : ' '
                   ;                   

fragment WORD_NC   : ~[ \n\r\t():]+
                   ;

ACTOR_NAME         : WORD_COL0 (SPACE+? WORD_NC)* SPACE*? COLON
                   ;                   

// Does not allow ':'
WORD_COL0          : {self.column == 0}? WORD_NC
                   ;

// Allows ':'
WORD               : {self.column != 0}? ~[ \n\r\t()]+
                   ;

PUNCT              : [.,?!] 
                   ;

COLON              : ':'
                   ;

PARENS_OPEN_COL0   : {self.column == 0}? '('
                   ;

PARENS_OPEN        : {self.column != 0}? '('
                   ;

PARENS_CLOSE       : ')'
                   ;

// Empty line is syntactically relevant so not skipping it.
// All can be clearned up after parse.
CR                 : [\r\n]+
                   ;

WS                 : [ \t]+ -> skip
                   ;

EMPTY_LINE         : (CR WS* CR)+
                   ;

```