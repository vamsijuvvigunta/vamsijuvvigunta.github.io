from dataclasses import dataclass
from abc import ABC, abstractmethod
from pydantic import BaseModel, computed_field
from typing import Optional, List

from antlrgen.ScreenplayListener import ScreenplayListener
from antlrgen.ScreenplayLexer import ScreenplayLexer
from antlrgen.ScreenplayParser import *

#----------------------------------------------------------------------------
# TODO:
#   Inside a listener callback with multiple possible parent-rules (like section_line)
#   I can either check for the type on the stack or expect them to have suitable
#   methods. Thinking thus:
#
#   class SectionLineConsumer(ABC):
#     def consume_section_line(str)
#
#   class ActionSection(BaseModel, SectionLineConsumer)
#   class SceneSection (BaseModel, SectionLineConsumer)
#----------------------------------------------------------------------------
class SectionLineConsumer(ABC):
    @abstractmethod
    def consume_section_line(self, section_line: str):
        pass        

# derived base-model which will contain a typename
# that is injected into all parents 
class TypeFieldBaseModel(BaseModel):
    @computed_field
    @property
    def type(self) -> str:
        """A computed property that concatenates first and last names."""
        return type(self).__name__

class SectionLines(TypeFieldBaseModel, SectionLineConsumer):
    content: list[str]

    def consume_section_line(self, section_line: str):
        self.content.append(section_line)

class SceneSection(TypeFieldBaseModel, SectionLineConsumer):
    content: SectionLines

    def consume_section_line(self, section_line: str):
        self.content.consume_section_line(section_line)

class ActorSceneSection(SceneSection):
    pass

class ActorSection(TypeFieldBaseModel, SectionLineConsumer):
    name   : str
    content: List[SectionLines | ActorSceneSection]    

    def consume_section_line(self, section_line: str):
        if len(self.content) == 0:
            self.content.append(SectionLines(content=[]))
        
        self.content[-1].consume_section_line(section_line)

    # called when a new ActorSceneSection is added to this
    # and we need to prepare for new upcoming sectionLines
    def append_new_section_lines_container(self):
        self.content.append(
            SectionLines(content=[])
        )

class ScreenPlay(TypeFieldBaseModel):    
    sections: List[ActorSection | SceneSection]

#----------------------------------------------------------------------------
# Subclass the listener to convert the AST into out dataclass heirarchy.
# Example at https://yetanotherprogrammingblog.medium.com/antlr-with-python-974c756bdb1b
class ScreenplayASTToDataclass(ScreenplayListener):

    def __init__(self):
        # Keep a stack with whatever object is being
        # built at the current parse level
        self.stack : List[any] = []
        self.parsed_screenplay = None        

    def parsed_data(self):
        return self.parsed_screenplay
    
    #- Stack management ------------
    def _pop(self):
        return self.stack.pop()

    def _push(self, obj):
        self.stack.append(obj)

    def _peek(self):
        return self.stack[-1]
    #-------------- Stack management -

    # Enter a parse tree produced by ScreenplayParser#screenplay.
    def enterScreenplay(self, ctx:ScreenplayParser.ScreenplayContext):
        self._push(
            ScreenPlay(sections=[])
            )

    # Exit a parse tree produced by ScreenplayParser#screenplay.
    def exitScreenplay(self, ctx:ScreenplayParser.ScreenplayContext):
        self.parsed_screenplay = self._pop()         
        assert len(self.stack) == 0

    # Enter a parse tree produced by ScreenplayParser#actor_section.    
    def enterActor_section(self, ctx:ScreenplayParser.Actor_sectionContext):
        self._push(
            ActorSection(name='pending', content=[])
        )        

    # Exit a parse tree produced by ScreenplayParser#actor_section.
    def exitActor_section(self, ctx:ScreenplayParser.Actor_sectionContext):
        actor_section = self._pop()

        top = self._peek()
        assert isinstance(top, ScreenPlay)
        top.sections.append(actor_section)

    # Enter a parse tree produced by ScreenplayParser#actor_name.
    def enterActor_name(self, ctx:ScreenplayParser.Actor_nameContext):                
        token = ctx.ACTOR_NAME()
        name  = token.getText()        
        
        # debug
        #print(f"actor_name. WORD token= {token}")
        #print(f"token.getText() = {token.getText()}")
        #print(f"token.getSymbol() = {token.getSymbol()}")
        #print(f"token.getChildCount() = {token.getChildCount()}")

        top = self._peek()
        assert isinstance(top, ActorSection)
        top.name = name

    # Exit a parse tree produced by ScreenplayParser#actor_name.
    def exitActor_name(self, ctx:ScreenplayParser.Actor_nameContext):
        # Nothing to pop as enterActor_name directly modifies the 
        # actorSection on the stack.
        pass


    # Enter a parse tree produced by ScreenplayParser#section_line_parens.
    def enterSection_line_parens(self, ctx:ScreenplayParser.Section_line_parensContext):
        # Join to put ' ' as separator. Otw, ctx.getText() couldbe been used
        section_line_txt = ' '.join([t.getText().strip() for t in ctx.getChildren()])
        self._peek().consume_section_line(section_line_txt)

    # Exit a parse tree produced by ScreenplayParser#section_line_parens.
    def exitSection_line_parens(self, ctx:ScreenplayParser.Section_line_parensContext):
        pass

    # Enter a parse tree produced by ScreenplayParser#section_line.
    #   section_line : WS? (WORD WS? PUNCT? WS?)+
    def enterSection_line(self, ctx:ScreenplayParser.Section_lineContext):                
        section_line_txt = ' '.join([t.getText().strip() for t in ctx.getChildren()])
        self._peek().consume_section_line(section_line_txt)        

    # Exit a parse tree produced by ScreenplayParser#section_line.
    def exitSection_line(self, ctx:ScreenplayParser.Section_lineContext):
        pass


    # Enter a parse tree produced by ScreenplayParser#scene_section.
    def enterScene_section(self, ctx:ScreenplayParser.Scene_sectionContext):
        self._push(
            SceneSection(
                content=SectionLines(content=[])
                )
        )        

    # Exit a parse tree produced by ScreenplayParser#scene_section.
    def exitScene_section(self, ctx:ScreenplayParser.Scene_sectionContext):
        scene_section = self._pop()

        top = self._peek()
        assert isinstance(top, ScreenPlay)
        top.sections.append(scene_section)

    # Enter a parse tree produced by ScreenplayParser#actor_scene_section.
    def enterActor_scene_section(self, ctx:ScreenplayParser.Actor_scene_sectionContext):        
        self._push(
            ActorSceneSection(
                content=SectionLines(content=[])
                )
        )                

    # Exit a parse tree produced by ScreenplayParser#actor_scene_section.
    def exitActor_scene_section(self, ctx:ScreenplayParser.Actor_scene_sectionContext):
        actor_scene_section = self._pop()

        top = self._peek()
        assert isinstance(top, ActorSection)
        top.content.append(actor_scene_section)

        # There is no rule for resuming regular actor_section once a actor_scene_section
        # ends. 
        # Assume the next tokens form regular sceneLines and add a new scene_lines object
        top.append_new_section_lines_container()