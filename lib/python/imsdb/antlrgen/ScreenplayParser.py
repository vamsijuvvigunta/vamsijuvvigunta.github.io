# Generated from Screenplay.g4 by ANTLR 4.13.2
# encoding: utf-8
from antlr4 import *
from io import StringIO
import sys
if sys.version_info[1] > 5:
	from typing import TextIO
else:
	from typing.io import TextIO

def serializedATN():
    return [
        4,1,11,106,2,0,7,0,2,1,7,1,2,2,7,2,2,3,7,3,2,4,7,4,2,5,7,5,2,6,7,
        6,1,0,1,0,5,0,17,8,0,10,0,12,0,20,9,0,1,0,1,0,5,0,24,8,0,10,0,12,
        0,27,9,0,5,0,29,8,0,10,0,12,0,32,9,0,1,0,1,0,1,1,1,1,1,1,3,1,39,
        8,1,1,1,3,1,42,8,1,4,1,44,8,1,11,1,12,1,45,1,2,1,2,1,3,4,3,51,8,
        3,11,3,12,3,52,1,4,4,4,56,8,4,11,4,12,4,57,1,4,1,4,5,4,62,8,4,10,
        4,12,4,65,9,4,1,4,3,4,68,8,4,4,4,70,8,4,11,4,12,4,71,1,5,1,5,1,5,
        3,5,77,8,5,4,5,79,8,5,11,5,12,5,80,1,5,1,5,1,5,1,5,3,5,87,8,5,4,
        5,89,8,5,11,5,12,5,90,3,5,93,8,5,1,6,1,6,1,6,3,6,98,8,6,4,6,100,
        8,6,11,6,12,6,101,1,6,1,6,1,6,0,0,7,0,2,4,6,8,10,12,0,2,2,0,9,9,
        11,11,1,0,8,8,118,0,30,1,0,0,0,2,35,1,0,0,0,4,47,1,0,0,0,6,50,1,
        0,0,0,8,69,1,0,0,0,10,92,1,0,0,0,12,94,1,0,0,0,14,18,3,2,1,0,15,
        17,7,0,0,0,16,15,1,0,0,0,17,20,1,0,0,0,18,16,1,0,0,0,18,19,1,0,0,
        0,19,29,1,0,0,0,20,18,1,0,0,0,21,25,3,10,5,0,22,24,7,0,0,0,23,22,
        1,0,0,0,24,27,1,0,0,0,25,23,1,0,0,0,25,26,1,0,0,0,26,29,1,0,0,0,
        27,25,1,0,0,0,28,14,1,0,0,0,28,21,1,0,0,0,29,32,1,0,0,0,30,28,1,
        0,0,0,30,31,1,0,0,0,31,33,1,0,0,0,32,30,1,0,0,0,33,34,5,0,0,1,34,
        1,1,0,0,0,35,43,3,4,2,0,36,39,3,6,3,0,37,39,3,12,6,0,38,36,1,0,0,
        0,38,37,1,0,0,0,39,41,1,0,0,0,40,42,5,9,0,0,41,40,1,0,0,0,41,42,
        1,0,0,0,42,44,1,0,0,0,43,38,1,0,0,0,44,45,1,0,0,0,45,43,1,0,0,0,
        45,46,1,0,0,0,46,3,1,0,0,0,47,48,5,1,0,0,48,5,1,0,0,0,49,51,5,3,
        0,0,50,49,1,0,0,0,51,52,1,0,0,0,52,50,1,0,0,0,52,53,1,0,0,0,53,7,
        1,0,0,0,54,56,5,3,0,0,55,54,1,0,0,0,56,57,1,0,0,0,57,55,1,0,0,0,
        57,58,1,0,0,0,58,70,1,0,0,0,59,63,5,7,0,0,60,62,8,1,0,0,61,60,1,
        0,0,0,62,65,1,0,0,0,63,61,1,0,0,0,63,64,1,0,0,0,64,67,1,0,0,0,65,
        63,1,0,0,0,66,68,5,8,0,0,67,66,1,0,0,0,67,68,1,0,0,0,68,70,1,0,0,
        0,69,55,1,0,0,0,69,59,1,0,0,0,70,71,1,0,0,0,71,69,1,0,0,0,71,72,
        1,0,0,0,72,9,1,0,0,0,73,78,5,6,0,0,74,76,3,8,4,0,75,77,5,9,0,0,76,
        75,1,0,0,0,76,77,1,0,0,0,77,79,1,0,0,0,78,74,1,0,0,0,79,80,1,0,0,
        0,80,78,1,0,0,0,80,81,1,0,0,0,81,82,1,0,0,0,82,83,5,8,0,0,83,93,
        1,0,0,0,84,86,3,8,4,0,85,87,5,9,0,0,86,85,1,0,0,0,86,87,1,0,0,0,
        87,89,1,0,0,0,88,84,1,0,0,0,89,90,1,0,0,0,90,88,1,0,0,0,90,91,1,
        0,0,0,91,93,1,0,0,0,92,73,1,0,0,0,92,88,1,0,0,0,93,11,1,0,0,0,94,
        99,5,7,0,0,95,97,3,8,4,0,96,98,5,9,0,0,97,96,1,0,0,0,97,98,1,0,0,
        0,98,100,1,0,0,0,99,95,1,0,0,0,100,101,1,0,0,0,101,99,1,0,0,0,101,
        102,1,0,0,0,102,103,1,0,0,0,103,104,5,8,0,0,104,13,1,0,0,0,20,18,
        25,28,30,38,41,45,52,57,63,67,69,71,76,80,86,90,92,97,101
    ]

class ScreenplayParser ( Parser ):

    grammarFileName = "Screenplay.g4"

    atn = ATNDeserializer().deserialize(serializedATN())

    decisionsToDFA = [ DFA(ds, i) for i, ds in enumerate(atn.decisionToState) ]

    sharedContextCache = PredictionContextCache()

    literalNames = [ "<INVALID>", "<INVALID>", "<INVALID>", "<INVALID>", 
                     "<INVALID>", "':'", "<INVALID>", "<INVALID>", "')'" ]

    symbolicNames = [ "<INVALID>", "ACTOR_NAME", "WORD_COL0", "WORD", "PUNCT", 
                      "COLON", "PARENS_OPEN_COL0", "PARENS_OPEN", "PARENS_CLOSE", 
                      "CR", "WS", "EMPTY_LINE" ]

    RULE_screenplay = 0
    RULE_actor_section = 1
    RULE_actor_name = 2
    RULE_section_line = 3
    RULE_section_line_parens = 4
    RULE_scene_section = 5
    RULE_actor_scene_section = 6

    ruleNames =  [ "screenplay", "actor_section", "actor_name", "section_line", 
                   "section_line_parens", "scene_section", "actor_scene_section" ]

    EOF = Token.EOF
    ACTOR_NAME=1
    WORD_COL0=2
    WORD=3
    PUNCT=4
    COLON=5
    PARENS_OPEN_COL0=6
    PARENS_OPEN=7
    PARENS_CLOSE=8
    CR=9
    WS=10
    EMPTY_LINE=11

    def __init__(self, input:TokenStream, output:TextIO = sys.stdout):
        super().__init__(input, output)
        self.checkVersion("4.13.2")
        self._interp = ParserATNSimulator(self, self.atn, self.decisionsToDFA, self.sharedContextCache)
        self._predicates = None




    class ScreenplayContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def EOF(self):
            return self.getToken(ScreenplayParser.EOF, 0)

        def actor_section(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(ScreenplayParser.Actor_sectionContext)
            else:
                return self.getTypedRuleContext(ScreenplayParser.Actor_sectionContext,i)


        def scene_section(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(ScreenplayParser.Scene_sectionContext)
            else:
                return self.getTypedRuleContext(ScreenplayParser.Scene_sectionContext,i)


        def CR(self, i:int=None):
            if i is None:
                return self.getTokens(ScreenplayParser.CR)
            else:
                return self.getToken(ScreenplayParser.CR, i)

        def EMPTY_LINE(self, i:int=None):
            if i is None:
                return self.getTokens(ScreenplayParser.EMPTY_LINE)
            else:
                return self.getToken(ScreenplayParser.EMPTY_LINE, i)

        def getRuleIndex(self):
            return ScreenplayParser.RULE_screenplay

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterScreenplay" ):
                listener.enterScreenplay(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitScreenplay" ):
                listener.exitScreenplay(self)




    def screenplay(self):

        localctx = ScreenplayParser.ScreenplayContext(self, self._ctx, self.state)
        self.enterRule(localctx, 0, self.RULE_screenplay)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 30
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while (((_la) & ~0x3f) == 0 and ((1 << _la) & 202) != 0):
                self.state = 28
                self._errHandler.sync(self)
                token = self._input.LA(1)
                if token in [1]:
                    self.state = 14
                    self.actor_section()
                    self.state = 18
                    self._errHandler.sync(self)
                    _la = self._input.LA(1)
                    while _la==9 or _la==11:
                        self.state = 15
                        _la = self._input.LA(1)
                        if not(_la==9 or _la==11):
                            self._errHandler.recoverInline(self)
                        else:
                            self._errHandler.reportMatch(self)
                            self.consume()
                        self.state = 20
                        self._errHandler.sync(self)
                        _la = self._input.LA(1)

                    pass
                elif token in [3, 6, 7]:
                    self.state = 21
                    self.scene_section()
                    self.state = 25
                    self._errHandler.sync(self)
                    _la = self._input.LA(1)
                    while _la==9 or _la==11:
                        self.state = 22
                        _la = self._input.LA(1)
                        if not(_la==9 or _la==11):
                            self._errHandler.recoverInline(self)
                        else:
                            self._errHandler.reportMatch(self)
                            self.consume()
                        self.state = 27
                        self._errHandler.sync(self)
                        _la = self._input.LA(1)

                    pass
                else:
                    raise NoViableAltException(self)

                self.state = 32
                self._errHandler.sync(self)
                _la = self._input.LA(1)

            self.state = 33
            self.match(ScreenplayParser.EOF)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Actor_sectionContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def actor_name(self):
            return self.getTypedRuleContext(ScreenplayParser.Actor_nameContext,0)


        def section_line(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(ScreenplayParser.Section_lineContext)
            else:
                return self.getTypedRuleContext(ScreenplayParser.Section_lineContext,i)


        def actor_scene_section(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(ScreenplayParser.Actor_scene_sectionContext)
            else:
                return self.getTypedRuleContext(ScreenplayParser.Actor_scene_sectionContext,i)


        def CR(self, i:int=None):
            if i is None:
                return self.getTokens(ScreenplayParser.CR)
            else:
                return self.getToken(ScreenplayParser.CR, i)

        def getRuleIndex(self):
            return ScreenplayParser.RULE_actor_section

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterActor_section" ):
                listener.enterActor_section(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitActor_section" ):
                listener.exitActor_section(self)




    def actor_section(self):

        localctx = ScreenplayParser.Actor_sectionContext(self, self._ctx, self.state)
        self.enterRule(localctx, 2, self.RULE_actor_section)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 35
            self.actor_name()
            self.state = 43 
            self._errHandler.sync(self)
            _alt = 1
            while _alt!=2 and _alt!=ATN.INVALID_ALT_NUMBER:
                if _alt == 1:
                    self.state = 38
                    self._errHandler.sync(self)
                    token = self._input.LA(1)
                    if token in [3]:
                        self.state = 36
                        self.section_line()
                        pass
                    elif token in [7]:
                        self.state = 37
                        self.actor_scene_section()
                        pass
                    else:
                        raise NoViableAltException(self)

                    self.state = 41
                    self._errHandler.sync(self)
                    la_ = self._interp.adaptivePredict(self._input,5,self._ctx)
                    if la_ == 1:
                        self.state = 40
                        self.match(ScreenplayParser.CR)



                else:
                    raise NoViableAltException(self)
                self.state = 45 
                self._errHandler.sync(self)
                _alt = self._interp.adaptivePredict(self._input,6,self._ctx)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Actor_nameContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def ACTOR_NAME(self):
            return self.getToken(ScreenplayParser.ACTOR_NAME, 0)

        def getRuleIndex(self):
            return ScreenplayParser.RULE_actor_name

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterActor_name" ):
                listener.enterActor_name(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitActor_name" ):
                listener.exitActor_name(self)




    def actor_name(self):

        localctx = ScreenplayParser.Actor_nameContext(self, self._ctx, self.state)
        self.enterRule(localctx, 4, self.RULE_actor_name)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 47
            self.match(ScreenplayParser.ACTOR_NAME)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Section_lineContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def WORD(self, i:int=None):
            if i is None:
                return self.getTokens(ScreenplayParser.WORD)
            else:
                return self.getToken(ScreenplayParser.WORD, i)

        def getRuleIndex(self):
            return ScreenplayParser.RULE_section_line

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterSection_line" ):
                listener.enterSection_line(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitSection_line" ):
                listener.exitSection_line(self)




    def section_line(self):

        localctx = ScreenplayParser.Section_lineContext(self, self._ctx, self.state)
        self.enterRule(localctx, 6, self.RULE_section_line)
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 50 
            self._errHandler.sync(self)
            _alt = 1
            while _alt!=2 and _alt!=ATN.INVALID_ALT_NUMBER:
                if _alt == 1:
                    self.state = 49
                    self.match(ScreenplayParser.WORD)

                else:
                    raise NoViableAltException(self)
                self.state = 52 
                self._errHandler.sync(self)
                _alt = self._interp.adaptivePredict(self._input,7,self._ctx)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Section_line_parensContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def PARENS_OPEN(self, i:int=None):
            if i is None:
                return self.getTokens(ScreenplayParser.PARENS_OPEN)
            else:
                return self.getToken(ScreenplayParser.PARENS_OPEN, i)

        def WORD(self, i:int=None):
            if i is None:
                return self.getTokens(ScreenplayParser.WORD)
            else:
                return self.getToken(ScreenplayParser.WORD, i)

        def PARENS_CLOSE(self, i:int=None):
            if i is None:
                return self.getTokens(ScreenplayParser.PARENS_CLOSE)
            else:
                return self.getToken(ScreenplayParser.PARENS_CLOSE, i)

        def getRuleIndex(self):
            return ScreenplayParser.RULE_section_line_parens

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterSection_line_parens" ):
                listener.enterSection_line_parens(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitSection_line_parens" ):
                listener.exitSection_line_parens(self)




    def section_line_parens(self):

        localctx = ScreenplayParser.Section_line_parensContext(self, self._ctx, self.state)
        self.enterRule(localctx, 8, self.RULE_section_line_parens)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 69 
            self._errHandler.sync(self)
            _alt = 1
            while _alt!=2 and _alt!=ATN.INVALID_ALT_NUMBER:
                if _alt == 1:
                    self.state = 69
                    self._errHandler.sync(self)
                    token = self._input.LA(1)
                    if token in [3]:
                        self.state = 55 
                        self._errHandler.sync(self)
                        _alt = 1
                        while _alt!=2 and _alt!=ATN.INVALID_ALT_NUMBER:
                            if _alt == 1:
                                self.state = 54
                                self.match(ScreenplayParser.WORD)

                            else:
                                raise NoViableAltException(self)
                            self.state = 57 
                            self._errHandler.sync(self)
                            _alt = self._interp.adaptivePredict(self._input,8,self._ctx)

                        pass
                    elif token in [7]:
                        self.state = 59
                        self.match(ScreenplayParser.PARENS_OPEN)
                        self.state = 63
                        self._errHandler.sync(self)
                        _alt = self._interp.adaptivePredict(self._input,9,self._ctx)
                        while _alt!=2 and _alt!=ATN.INVALID_ALT_NUMBER:
                            if _alt==1:
                                self.state = 60
                                _la = self._input.LA(1)
                                if _la <= 0 or _la==8:
                                    self._errHandler.recoverInline(self)
                                else:
                                    self._errHandler.reportMatch(self)
                                    self.consume() 
                            self.state = 65
                            self._errHandler.sync(self)
                            _alt = self._interp.adaptivePredict(self._input,9,self._ctx)

                        self.state = 67
                        self._errHandler.sync(self)
                        la_ = self._interp.adaptivePredict(self._input,10,self._ctx)
                        if la_ == 1:
                            self.state = 66
                            self.match(ScreenplayParser.PARENS_CLOSE)


                        pass
                    else:
                        raise NoViableAltException(self)


                else:
                    raise NoViableAltException(self)
                self.state = 71 
                self._errHandler.sync(self)
                _alt = self._interp.adaptivePredict(self._input,12,self._ctx)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Scene_sectionContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def PARENS_OPEN_COL0(self):
            return self.getToken(ScreenplayParser.PARENS_OPEN_COL0, 0)

        def PARENS_CLOSE(self):
            return self.getToken(ScreenplayParser.PARENS_CLOSE, 0)

        def section_line_parens(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(ScreenplayParser.Section_line_parensContext)
            else:
                return self.getTypedRuleContext(ScreenplayParser.Section_line_parensContext,i)


        def CR(self, i:int=None):
            if i is None:
                return self.getTokens(ScreenplayParser.CR)
            else:
                return self.getToken(ScreenplayParser.CR, i)

        def getRuleIndex(self):
            return ScreenplayParser.RULE_scene_section

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterScene_section" ):
                listener.enterScene_section(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitScene_section" ):
                listener.exitScene_section(self)




    def scene_section(self):

        localctx = ScreenplayParser.Scene_sectionContext(self, self._ctx, self.state)
        self.enterRule(localctx, 10, self.RULE_scene_section)
        self._la = 0 # Token type
        try:
            self.state = 92
            self._errHandler.sync(self)
            token = self._input.LA(1)
            if token in [6]:
                self.enterOuterAlt(localctx, 1)
                self.state = 73
                self.match(ScreenplayParser.PARENS_OPEN_COL0)
                self.state = 78 
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                while True:
                    self.state = 74
                    self.section_line_parens()
                    self.state = 76
                    self._errHandler.sync(self)
                    _la = self._input.LA(1)
                    if _la==9:
                        self.state = 75
                        self.match(ScreenplayParser.CR)


                    self.state = 80 
                    self._errHandler.sync(self)
                    _la = self._input.LA(1)
                    if not (_la==3 or _la==7):
                        break

                self.state = 82
                self.match(ScreenplayParser.PARENS_CLOSE)
                pass
            elif token in [3, 7]:
                self.enterOuterAlt(localctx, 2)
                self.state = 88 
                self._errHandler.sync(self)
                _alt = 1
                while _alt!=2 and _alt!=ATN.INVALID_ALT_NUMBER:
                    if _alt == 1:
                        self.state = 84
                        self.section_line_parens()
                        self.state = 86
                        self._errHandler.sync(self)
                        la_ = self._interp.adaptivePredict(self._input,15,self._ctx)
                        if la_ == 1:
                            self.state = 85
                            self.match(ScreenplayParser.CR)



                    else:
                        raise NoViableAltException(self)
                    self.state = 90 
                    self._errHandler.sync(self)
                    _alt = self._interp.adaptivePredict(self._input,16,self._ctx)

                pass
            else:
                raise NoViableAltException(self)

        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx


    class Actor_scene_sectionContext(ParserRuleContext):
        __slots__ = 'parser'

        def __init__(self, parser, parent:ParserRuleContext=None, invokingState:int=-1):
            super().__init__(parent, invokingState)
            self.parser = parser

        def PARENS_OPEN(self):
            return self.getToken(ScreenplayParser.PARENS_OPEN, 0)

        def PARENS_CLOSE(self):
            return self.getToken(ScreenplayParser.PARENS_CLOSE, 0)

        def section_line_parens(self, i:int=None):
            if i is None:
                return self.getTypedRuleContexts(ScreenplayParser.Section_line_parensContext)
            else:
                return self.getTypedRuleContext(ScreenplayParser.Section_line_parensContext,i)


        def CR(self, i:int=None):
            if i is None:
                return self.getTokens(ScreenplayParser.CR)
            else:
                return self.getToken(ScreenplayParser.CR, i)

        def getRuleIndex(self):
            return ScreenplayParser.RULE_actor_scene_section

        def enterRule(self, listener:ParseTreeListener):
            if hasattr( listener, "enterActor_scene_section" ):
                listener.enterActor_scene_section(self)

        def exitRule(self, listener:ParseTreeListener):
            if hasattr( listener, "exitActor_scene_section" ):
                listener.exitActor_scene_section(self)




    def actor_scene_section(self):

        localctx = ScreenplayParser.Actor_scene_sectionContext(self, self._ctx, self.state)
        self.enterRule(localctx, 12, self.RULE_actor_scene_section)
        self._la = 0 # Token type
        try:
            self.enterOuterAlt(localctx, 1)
            self.state = 94
            self.match(ScreenplayParser.PARENS_OPEN)
            self.state = 99 
            self._errHandler.sync(self)
            _la = self._input.LA(1)
            while True:
                self.state = 95
                self.section_line_parens()
                self.state = 97
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if _la==9:
                    self.state = 96
                    self.match(ScreenplayParser.CR)


                self.state = 101 
                self._errHandler.sync(self)
                _la = self._input.LA(1)
                if not (_la==3 or _la==7):
                    break

            self.state = 103
            self.match(ScreenplayParser.PARENS_CLOSE)
        except RecognitionException as re:
            localctx.exception = re
            self._errHandler.reportError(self, re)
            self._errHandler.recover(self, re)
        finally:
            self.exitRule()
        return localctx





