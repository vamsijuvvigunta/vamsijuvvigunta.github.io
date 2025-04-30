# See base layers being built up to reduce layer size
# infrastructure/docker-images/hillops_inference_kserve_base
FROM hillops_inference_kserve_base:py3.10_cuda118_ks0.11

COPY model model

WORKDIR model

#RUN pip install --upgrade pip && pip install -r ./requirements.txt

# Unbuffered output for logging ?
ENTRYPOINT ["python", "-u", "KServeBertSentimentModel.py"]
