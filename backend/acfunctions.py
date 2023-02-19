import os, json, time, sys

# convert seconds into timecode
hhmmss = lambda x: time.strftime('%H:%M:%S', time.gmtime(float(x)))


def get_bookmarks():
    f = open('bookmarks.json')
    bookmarks = json.load(f)
    return bookmarks["bookmarks"]


def convert(file, key, out=os.getcwd()):
    name = os.path.basename(file).split('.')[0].split('_ep')[0]
    if out[-1] != '/': out += '/'
    outfile = out + name + '/'

    try:
        os.mkdir(outfile)
    except Exception as e:
        print(name, ' already exists\n --- SKIPPING ---')
        return None


    bookmarks = get_bookmarks()

    def writeBookmark(b, index):
        bid = index if index > 9 else "0" + str(index)
        chapter_end_pos = 1000000000000 if b["endPositionInMs"] is None else b["endPositionInMs"]
        start_time = max((b["chapterPositionInMs"] / 1000) - 30, 0)
        end_time = min((b["chapterPositionInMs"] / 1000) + 30, chapter_end_pos)
        chapter_name = b["chapterName"]
        os.popen(
            f'ffmpeg -activation_bytes {key} -ss {hhmmss(start_time)} -to {hhmmss(end_time)} -i {file} -codec:a flac {outfile}b{bid}_{name}.flac -metadata title="{chapter_name}" -y -loglevel quiet -stats')

    for idx, x in enumerate(bookmarks):
        writeBookmark(x, idx)

    print('Finished converting ' + name)
    return outfile
