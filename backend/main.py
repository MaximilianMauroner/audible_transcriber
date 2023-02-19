try:
    from . import acfunctions as fns
except ImportError:
    import acfunctions as fns
import glob, os, sys

try:
    from . import transcription as tr
except ImportError:
    import transcription as tr

try:
    from . import transofm_transcript as tt
except ImportError:
    import transofm_transcript as tt

key = False

dir = os.path.dirname("audible_files/")
if dir[-1] != '/': dir += '/'
files = glob.glob(dir + '*.aax')
print(sys.argv, dir, len(files))

out = os.path.dirname('bookmarks/')

skip = True
if not key and not skip:
    try:
        from . import activation
    except ImportError:
        import activation
    key = activation.get_key(files[0])

for file in files:

    print('Converting file:', file)
    outfolder = "bookmarks/HowtoTalktoAnyone92LittleTricksforBigSuccessinRelationships/"
    if not skip:
        outfolder = fns.convert(file, key, out)

    try:
        os.mkdir(outfolder + "transcripts")
    except Exception as e:
        print("transcripts folder", ' already exists\n --- SKIPPING ---')
    # print(outfolder)
    # tr.transcribe_folder(outfolder)
    tt.transcribe_folder(outfolder)
