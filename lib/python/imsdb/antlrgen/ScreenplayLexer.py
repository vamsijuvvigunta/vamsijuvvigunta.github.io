# Generated from Screenplay.g4 by ANTLR 4.13.2
from antlr4 import *
from io import StringIO
import sys
if sys.version_info[1] > 5:
    from typing import TextIO
else:
    from typing.io import TextIO


def serializedATN():
    return [
        4,0,11,105,6,-1,2,0,7,0,2,1,7,1,2,2,7,2,2,3,7,3,2,4,7,4,2,5,7,5,
        2,6,7,6,2,7,7,7,2,8,7,8,2,9,7,9,2,10,7,10,2,11,7,11,2,12,7,12,2,
        13,7,13,1,0,1,0,1,1,1,1,1,2,4,2,35,8,2,11,2,12,2,36,1,3,1,3,4,3,
        41,8,3,11,3,12,3,42,1,3,1,3,5,3,47,8,3,10,3,12,3,50,9,3,1,3,5,3,
        53,8,3,10,3,12,3,56,9,3,1,3,1,3,1,4,1,4,1,4,1,5,1,5,4,5,65,8,5,11,
        5,12,5,66,1,6,1,6,1,7,1,7,1,8,1,8,1,8,1,9,1,9,1,9,1,10,1,10,1,11,
        4,11,82,8,11,11,11,12,11,83,1,12,4,12,87,8,12,11,12,12,12,88,1,12,
        1,12,1,13,1,13,5,13,95,8,13,10,13,12,13,98,9,13,1,13,1,13,4,13,102,
        8,13,11,13,12,13,103,2,42,54,0,14,1,0,3,0,5,0,7,1,9,2,11,3,13,4,
        15,5,17,6,19,7,21,8,23,9,25,10,27,11,1,0,6,1,0,48,57,5,0,9,10,13,
        13,32,32,40,41,58,58,4,0,9,10,13,13,32,32,40,41,4,0,33,33,44,44,
        46,46,63,63,2,0,10,10,13,13,2,0,9,9,32,32,110,0,7,1,0,0,0,0,9,1,
        0,0,0,0,11,1,0,0,0,0,13,1,0,0,0,0,15,1,0,0,0,0,17,1,0,0,0,0,19,1,
        0,0,0,0,21,1,0,0,0,0,23,1,0,0,0,0,25,1,0,0,0,0,27,1,0,0,0,1,29,1,
        0,0,0,3,31,1,0,0,0,5,34,1,0,0,0,7,38,1,0,0,0,9,59,1,0,0,0,11,62,
        1,0,0,0,13,68,1,0,0,0,15,70,1,0,0,0,17,72,1,0,0,0,19,75,1,0,0,0,
        21,78,1,0,0,0,23,81,1,0,0,0,25,86,1,0,0,0,27,101,1,0,0,0,29,30,7,
        0,0,0,30,2,1,0,0,0,31,32,5,32,0,0,32,4,1,0,0,0,33,35,8,1,0,0,34,
        33,1,0,0,0,35,36,1,0,0,0,36,34,1,0,0,0,36,37,1,0,0,0,37,6,1,0,0,
        0,38,48,3,9,4,0,39,41,3,3,1,0,40,39,1,0,0,0,41,42,1,0,0,0,42,43,
        1,0,0,0,42,40,1,0,0,0,43,44,1,0,0,0,44,45,3,5,2,0,45,47,1,0,0,0,
        46,40,1,0,0,0,47,50,1,0,0,0,48,46,1,0,0,0,48,49,1,0,0,0,49,54,1,
        0,0,0,50,48,1,0,0,0,51,53,3,3,1,0,52,51,1,0,0,0,53,56,1,0,0,0,54,
        55,1,0,0,0,54,52,1,0,0,0,55,57,1,0,0,0,56,54,1,0,0,0,57,58,3,15,
        7,0,58,8,1,0,0,0,59,60,4,4,0,0,60,61,3,5,2,0,61,10,1,0,0,0,62,64,
        4,5,1,0,63,65,8,2,0,0,64,63,1,0,0,0,65,66,1,0,0,0,66,64,1,0,0,0,
        66,67,1,0,0,0,67,12,1,0,0,0,68,69,7,3,0,0,69,14,1,0,0,0,70,71,5,
        58,0,0,71,16,1,0,0,0,72,73,4,8,2,0,73,74,5,40,0,0,74,18,1,0,0,0,
        75,76,4,9,3,0,76,77,5,40,0,0,77,20,1,0,0,0,78,79,5,41,0,0,79,22,
        1,0,0,0,80,82,7,4,0,0,81,80,1,0,0,0,82,83,1,0,0,0,83,81,1,0,0,0,
        83,84,1,0,0,0,84,24,1,0,0,0,85,87,7,5,0,0,86,85,1,0,0,0,87,88,1,
        0,0,0,88,86,1,0,0,0,88,89,1,0,0,0,89,90,1,0,0,0,90,91,6,12,0,0,91,
        26,1,0,0,0,92,96,3,23,11,0,93,95,3,25,12,0,94,93,1,0,0,0,95,98,1,
        0,0,0,96,94,1,0,0,0,96,97,1,0,0,0,97,99,1,0,0,0,98,96,1,0,0,0,99,
        100,3,23,11,0,100,102,1,0,0,0,101,92,1,0,0,0,102,103,1,0,0,0,103,
        101,1,0,0,0,103,104,1,0,0,0,104,28,1,0,0,0,10,0,36,42,48,54,66,83,
        88,96,103,1,6,0,0
    ]

class ScreenplayLexer(Lexer):

    atn = ATNDeserializer().deserialize(serializedATN())

    decisionsToDFA = [ DFA(ds, i) for i, ds in enumerate(atn.decisionToState) ]

    ACTOR_NAME = 1
    WORD_COL0 = 2
    WORD = 3
    PUNCT = 4
    COLON = 5
    PARENS_OPEN_COL0 = 6
    PARENS_OPEN = 7
    PARENS_CLOSE = 8
    CR = 9
    WS = 10
    EMPTY_LINE = 11

    channelNames = [ u"DEFAULT_TOKEN_CHANNEL", u"HIDDEN" ]

    modeNames = [ "DEFAULT_MODE" ]

    literalNames = [ "<INVALID>",
            "':'", "')'" ]

    symbolicNames = [ "<INVALID>",
            "ACTOR_NAME", "WORD_COL0", "WORD", "PUNCT", "COLON", "PARENS_OPEN_COL0", 
            "PARENS_OPEN", "PARENS_CLOSE", "CR", "WS", "EMPTY_LINE" ]

    ruleNames = [ "DIGIT", "SPACE", "WORD_NC", "ACTOR_NAME", "WORD_COL0", 
                  "WORD", "PUNCT", "COLON", "PARENS_OPEN_COL0", "PARENS_OPEN", 
                  "PARENS_CLOSE", "CR", "WS", "EMPTY_LINE" ]

    grammarFileName = "Screenplay.g4"

    def __init__(self, input=None, output:TextIO = sys.stdout):
        super().__init__(input, output)
        self.checkVersion("4.13.2")
        self._interp = LexerATNSimulator(self, self.atn, self.decisionsToDFA, PredictionContextCache())
        self._actions = None
        self._predicates = None


    def sempred(self, localctx:RuleContext, ruleIndex:int, predIndex:int):
        if self._predicates is None:
            preds = dict()
            preds[4] = self.WORD_COL0_sempred
            preds[5] = self.WORD_sempred
            preds[8] = self.PARENS_OPEN_COL0_sempred
            preds[9] = self.PARENS_OPEN_sempred
            self._predicates = preds
        pred = self._predicates.get(ruleIndex, None)
        if pred is not None:
            return pred(localctx, predIndex)
        else:
            raise Exception("No registered predicate for:" + str(ruleIndex))

    def WORD_COL0_sempred(self, localctx:RuleContext, predIndex:int):
            if predIndex == 0:
                return self.column == 0
         

    def WORD_sempred(self, localctx:RuleContext, predIndex:int):
            if predIndex == 1:
                return self.column != 0
         

    def PARENS_OPEN_COL0_sempred(self, localctx:RuleContext, predIndex:int):
            if predIndex == 2:
                return self.column == 0
         

    def PARENS_OPEN_sempred(self, localctx:RuleContext, predIndex:int):
            if predIndex == 3:
                return self.column != 0
         


