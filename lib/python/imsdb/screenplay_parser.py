# Use example from..
# Except use the stream-from-text for temp writing

import sys
import json
from antlr4 import*
from antlrgen.ScreenplayLexer  import ScreenplayLexer
from antlrgen.ScreenplayParser import ScreenplayParser

from screenplay_json import ScreenplayASTToDataclass

# Sample texts in increasing order of complexity.
samples = {}
samples["sample_just_actors"] = """ALADDIN:    Sultan?  They want me to be sultan?

GENIE:  Huzzah!  Hail the conquering hero!  
        Aladdin, you've just won the heart of the princess.  
        What are you gonna do next?
"""

# Adding a scene between actor sections
samples["sample_actors_plus_scene"] = """ALADDIN:    Sultan?  They want me to be sultan?

(GENIE comes out of lamp)

GENIE:  Huzzah!  Hail the conquering hero! Aladdin, you've just won
        the heart of the princess.  What are you gonna do
        next?
        Psst, your line is "I'm going to free the genie."
        Anytime.
"""

# Adding (..) scene portions inside the "GENIE:" actor segment
samples["sample_actors_plus_scene_plus_actor_scene"] = """ALADDIN:    Sultan?  They want me to be sultan?

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

# Introducing "Whispering:"
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

# Introducing a nested (a 4 post bed)
samples["sample_actors_plus_scene_plus_actor_scene_plus_colonwords_plus_nested_parens"] = """ALADDIN:    Sultan?  They want me to be sultan?

(GENIE comes out of lamp)

GENIE:  Huzzah!  Hail the conquering hero!  (Turns into a
        one-man band.  He sees ALADDIN walk away with his
        head hung.  He stops, scratches his head, comes up
        with an idea, then zooms over to ALADDIN.  He holds
        up his hands like a director scoping a picture and
        we look through them.)  Aladdin, you've just won
        the heart of the princess.  What are you gonna do
        next?  (ALADDIN looks at him, then walks away in
        sadness to the bed (a 4 post bed), where he falls on it and sighs.
        GENIE again is confused, then goes to him and pulls
        out a script labeled "Aladdin." Whispering:)
        Psst, your line is "I'm going to free the genie."
        Anytime.
"""

# Introducing a nested (a 4 post bed)
samples["names_with_indices"] = """ALADDIN:    Sultan?  They want me to be sultan?

GUARD 1: Halt! Who goes there.
GUARD 2: Show yourself criminal!
GUARD 33: Yes, reveal thine criminality!!
"""

# Introducing song sections. Treat these also as scene_sections
# but without parens
samples["scene_section_without_parens"] = """ALADDIN:    Come on, let's get outta here!
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
"""

# Extended role naming
samples["extended_role_names"] = """ALADDIN and JASMINE: Some lines

OLD MAN: old is gold you know!
DUP. GENIES: Which of us is the original and which dup?
CHORUS OF MEN: Chorus to death
CHORUS OF WOMEN: Chorus em to death as well.
MEN'S CHORUS: Is there an echo in here??
"""

sample = samples["extended_role_names"]

def main(argv):    
    # Either FileStream if I have an arg or the local hardcoded sample.
    input_stream = FileStream(argv[1]) if len(argv) > 1 else InputStream(sample)    

    lexer  = ScreenplayLexer(input_stream)
    stream = CommonTokenStream(lexer)
    parser = ScreenplayParser(stream)

    tree = parser.screenplay()
    #print(tree.toStringTree(recog=parser))

    walker = ParseTreeWalker()
    converter = ScreenplayASTToDataclass()
    walker.walk(converter, tree)
    
    parsed_obj = converter.parsed_data()
    print(json.dumps(parsed_obj.model_dump(mode='json'), indent=4))

if __name__ == '__main__':
    main(sys.argv)