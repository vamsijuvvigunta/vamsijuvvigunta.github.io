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

