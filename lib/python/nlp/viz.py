import spacy
import nltk

# See hillops/nbs/NLP/NLP_AnnotationRendering.ipynb for related notebook
def get_displaycy_from_stanza_dependencies(tokens, enhPPDeps):
    """
    tokens: The list of tokens after an annotations. Expected to be a dict
            with keys 'text' and 'tag'.
    enhPPDeps: The enhancedPlusPlus dependencies from stanza. 

    use the return like this:
      from spacy import displacy
      tree = get_displaycy_from_stanza_dependencies(..)
      displacy.render(tree, style="dep", manual=True)

    enhPPDeps are typcally obtained (depparseEnhanced.sentence[0].enhancedPlusPlusDependencies) via

    from stanza.pipeline.core import DownloadMethod
    enh_ud_nlp = stanza.Pipeline(lang='en', processors='tokenize,pos,lemma,constituency,ner,depparse')
    with ud_enhancer.UniversalEnhancer(language="en") as enhancer:
      depparseFromStanza = enh_ud_nlp("John said that he loved mila")
      depparseEnhanced = enhancer.process(depparseFromStanza)
    """
    # See "Dep Input" section under https://spacy.io/usage/visualizers#manual-usage
    # for data format
    words = [ {'text':t.word, 'tag':t.pos} for t in tokens]

    arcs = []
    for dep_edge in enhPPDeps.edge:
        # From experimentation Spacy wants start < end
        # Spacy uses 0 indxed words unlike StanfordCoreNLP's 1 indexed words
        # and the dir to specify the actual direction of the arrow.
        if dep_edge.source < dep_edge.target:    
            arc = {
                'start' : dep_edge.source - 1,
                'end'   : dep_edge.target - 1,
                'label' : dep_edge.dep,
                'dir'   : 'right'
                }    
        else:
            arc = {
                'start' : dep_edge.target - 1,
                'end'   : dep_edge.source - 1,
                'label' : dep_edge.dep,
                'dir'   : 'left'
                }
        
        arcs.append(arc)

    return {'words' : words, 'arcs' : arcs}

