import os
import speech_recognition as sr

r = sr.Recognizer()

def transcribe_audio(folder, audio_file):
    print("Reading audio file: " + audio_file)
    with sr.AudioFile(folder + audio_file) as source:
        audio = r.record(source)
        audio_file = audio_file.split(".")[0]
        try:
            transcript = r.recognize_google(audio)
            save_transcript(transcript, folder + "transcripts/" + audio_file + ".txt")
        except sr.UnknownValueError:
            print("Google Speech Recognition could not understand audio")
        except sr.RequestError as e:
            print("Could not request results from Google Speech Recognition service; {0}".format(e))


def transcribe_folder(outfolder):
    for file in os.listdir(outfolder):
        if file.endswith(".flac"):
            try:
                transcribe_audio(outfolder, file)
            except Exception as e:
                print("Error: " + str(e))
            continue
        else:
            continue


def save_transcript(transcript, transcriptFileName):
    f = open(transcriptFileName, "w")
    f.write(transcript)
    f.close()
