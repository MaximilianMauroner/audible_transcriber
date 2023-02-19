import os
import librosa
import torch

from transformers import Wav2Vec2ForCTC, Wav2Vec2Tokenizer

# Load the pre-trained Wav2Vec2 model and tokenizer
model = Wav2Vec2ForCTC.from_pretrained("facebook/wav2vec2-large-960h-lv60-self")
tokenizer = Wav2Vec2Tokenizer.from_pretrained("facebook/wav2vec2-large-960h-lv60-self")


# Load an audio file and convert it to a feature vector

def transcribe(folder, audio_file):
    audio_input, sample_rate = librosa.load(folder + audio_file, sr=16000)
    inputs = tokenizer(audio_input, return_tensors="pt").input_values

    # Transcribe the audio using the Wav2Vec2 model
    with torch.no_grad():
        logits = model(inputs).logits
    predicted_ids = torch.argmax(logits, dim=-1)
    transcription = tokenizer.decode(predicted_ids[0])

    print(transcription)


def transcribe_folder(outfolder):
    for file in os.listdir(outfolder):
        if file.endswith(".flac") and file.startswith("b00"):
            try:
                transcribe(outfolder, file)
            except Exception as e:
                print("Error: " + str(e))
            continue
        else:
            continue
