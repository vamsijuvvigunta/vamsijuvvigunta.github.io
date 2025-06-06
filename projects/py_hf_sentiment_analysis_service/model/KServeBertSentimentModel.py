from typing import Dict

import os
import sys
import json
import kserve
import torch

from transformers import AutoModelForSequenceClassification, AutoTokenizer
from kserve import ModelServer

# Set logger to stdout
# The root-logger's level is already set outside.
# Can I set level just on this module logger ??
import logging

# Dig into base class if needed.
# Mostly copied https://medium.com/mlearning-ai/deploying-a-huggingface-bert-model-with-kserve-3e521d69e596
class KServeBERTSentimentModel(kserve.Model):

    def __init__(self, name: str):
        super().__init__(name)        

        # Logging
        _KSERVE_LOGGER_NAME='KServeBERTSentimentModel'
        self._setupLogging(_KSERVE_LOGGER_NAME)

        # Rest of the base-class
        self.name = name
        self.ready = False
    
    # Inherit same handlers as root but allow overriding of loglevel
    def _setupLogging(self, loggerName):        
        self.logger = logging.getLogger(loggerName)
        if "LOGLEVEL" in os.environ:
            self.logger.setLevel(os.getenv("LOGLEVEL").upper())

    def load(self):
        # build tokenizer and model
        # Search at https://huggingface.co/models and follow some HF examples/tutorials
        _NLP_MODEL_NAME = "distilbert-base-uncased-finetuned-sst-2-english"
        self.tokenizer = AutoTokenizer.from_pretrained(_NLP_MODEL_NAME)

        # Hmm.. converts to torchscript on the fly ?
        self.model = AutoModelForSequenceClassification.from_pretrained(
            _NLP_MODEL_NAME,
            torchscript = True
        )

        self.ready = True
    
    def predict(self, request: Dict, headers: Dict) -> Dict:

        # Dump the entire request out for debugging
        self.logger.debug(
            json.dumps(request,indent=4)
        )

        # Originally used _sequence = request["sequence"]
        # I Would not publish this type of unchecked key access into a medum article
        # unless there was some schema validation step infront.
        #
        # error stating that request is of type `bytes`.
        _sequence = request["sequence"]
        self.logger.info(f"sequence:--{_sequence}")

        inputs = self.tokenizer(
            _sequence,
            return_tensors="pt",  #?
            max_length=128,       #?
            padding="max_length", #?
            truncation=True,
        )

        # Run the prediction
        with torch.no_grad():
            predictions = self.model(**inputs)[0]
            scores = torch.nn.Softmax(dim=1)(predictions)
        
        _results = [{
            "label": self.model.config.id2label[item.argmax().item()],
            "score": item.max().item()}
            for item in scores]
        
        self.logger.info(f"results:-- {_results}")

        return {"predictions": _results}


if __name__ == "__main__":

    model = KServeBERTSentimentModel("bert-sentiment")
    model.load()

    model_server = ModelServer(http_port=8080, workers=1)
    model_server.start([model])
