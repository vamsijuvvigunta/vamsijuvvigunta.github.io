# Generated from Screenplay.g4 by ANTLR 4.13.2
from antlr4 import *
if "." in __name__:
    from .ScreenplayParser import ScreenplayParser
else:
    from ScreenplayParser import ScreenplayParser

# This class defines a complete listener for a parse tree produced by ScreenplayParser.
class ScreenplayListener(ParseTreeListener):

    # Enter a parse tree produced by ScreenplayParser#screenplay.
    def enterScreenplay(self, ctx:ScreenplayParser.ScreenplayContext):
        pass

    # Exit a parse tree produced by ScreenplayParser#screenplay.
    def exitScreenplay(self, ctx:ScreenplayParser.ScreenplayContext):
        pass


    # Enter a parse tree produced by ScreenplayParser#actor_section.
    def enterActor_section(self, ctx:ScreenplayParser.Actor_sectionContext):
        pass

    # Exit a parse tree produced by ScreenplayParser#actor_section.
    def exitActor_section(self, ctx:ScreenplayParser.Actor_sectionContext):
        pass


    # Enter a parse tree produced by ScreenplayParser#actor_name.
    def enterActor_name(self, ctx:ScreenplayParser.Actor_nameContext):
        pass

    # Exit a parse tree produced by ScreenplayParser#actor_name.
    def exitActor_name(self, ctx:ScreenplayParser.Actor_nameContext):
        pass


    # Enter a parse tree produced by ScreenplayParser#section_line.
    def enterSection_line(self, ctx:ScreenplayParser.Section_lineContext):
        pass

    # Exit a parse tree produced by ScreenplayParser#section_line.
    def exitSection_line(self, ctx:ScreenplayParser.Section_lineContext):
        pass


    # Enter a parse tree produced by ScreenplayParser#section_line_parens.
    def enterSection_line_parens(self, ctx:ScreenplayParser.Section_line_parensContext):
        pass

    # Exit a parse tree produced by ScreenplayParser#section_line_parens.
    def exitSection_line_parens(self, ctx:ScreenplayParser.Section_line_parensContext):
        pass


    # Enter a parse tree produced by ScreenplayParser#scene_section.
    def enterScene_section(self, ctx:ScreenplayParser.Scene_sectionContext):
        pass

    # Exit a parse tree produced by ScreenplayParser#scene_section.
    def exitScene_section(self, ctx:ScreenplayParser.Scene_sectionContext):
        pass


    # Enter a parse tree produced by ScreenplayParser#actor_scene_section.
    def enterActor_scene_section(self, ctx:ScreenplayParser.Actor_scene_sectionContext):
        pass

    # Exit a parse tree produced by ScreenplayParser#actor_scene_section.
    def exitActor_scene_section(self, ctx:ScreenplayParser.Actor_scene_sectionContext):
        pass



del ScreenplayParser