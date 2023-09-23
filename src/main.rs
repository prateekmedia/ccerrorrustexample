use clap::{arg, Parser};

fn main() {
    let _: Args = Args::parse();
}

#[derive(Debug, Parser)]
#[command(name = "CCExtractor")]
#[command(author = "Carlos Fernandez Sanz, Volker Quetschke.")]
#[command(version = "0.94")]
#[command(about = "Teletext portions taken from Petr Kutalek's telxcc
--------------------------------------------------------------------------
Originally based on McPoodle's tools. Check his page for lots of information
on closed captions technical details.
(http://www.theneitherworld.com/mcpoodle/SCC_TOOLS/DOCS/SCC_TOOLS.HTML)

This tool home page:
http://www.ccextractor.org
  Extracts closed captions and teletext subtitles from video streams.
    (DVB, .TS, ReplayTV 4000 and 5000, dvr-ms, bttv, Tivo, Dish Network,
     .mp4, HDHomeRun are known to work).

  Syntax:
  ccextractor [options] inputfile1 [inputfile2...] [-o outputfilename]
")]
#[command(
    help_template = "{name} {version}, {author}.\n{about}\n {all-args} {tab}\n
    An example command for burned-in subtitle extraction is as follows:
    ccextractor video.mp4 --hardsubx --subcolor white --detect_italics --whiteness_thresh 90 --conf_thresh 60
    
    Notes on the CEA-708 decoder: While it is starting to be useful, it's
    a work in progress. A number of things don't work yet in the decoder
    itself, and many of the auxiliary tools (case conversion to name one)
    won't do anything yet. Feel free to submit samples that cause problems
    and feature requests.

    Notes on spupng output format:
    One .xml file is created per output field. A set of .png files are created in
    a directory with the same base name as the corresponding .xml file(s), but with
    a .d extension. Each .png file will contain an image representing one caption
    and named subNNNN.png, starting with sub0000.png.
    For example, the command:
        ccextractor -out=spupng input.mpg
    will create the files:
        input.xml
        input.d/sub0000.png
        input.d/sub0001.png
        ...
    The command:
        ccextractor -out=spupng -o /tmp/output -12 input.mpg
    will create the files:
        /tmp/output_1.xml
        /tmp/output_1.d/sub0000.png
        /tmp/output_1.d/sub0001.png
        ...
        /tmp/output_2.xml
        /tmp/output_2.d/sub0000.png
        /tmp/output_2.d/sub0001.png
        ...
    "
)]
#[command(arg_required_else_help = true)]
pub struct Args {
    /// file(s) to process
    #[arg(value_name = "inputfile")]
    pub inputfile: Option<Vec<String>>,
    /// Use -o parameters to define output filename if you don't
    /// like the default ones (same as infile plus _1 or _2 when
    /// needed and file extension, e.g. .srt).
    #[arg(
        short,
        value_name = "outputfilename",
        verbatim_doc_comment,
        help_heading = "Heading"
    )]
    pub output: Option<String>,
    /// Write output to stdout (console) instead of file. If
    /// stdout is used, then -o can't be used. Also
    /// --stdout will redirect all messages to stderr (error).
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub stdout: bool,
    /// Dump the PES Header to stdout (console). This is
    /// used for debugging purposes to see the contents
    /// of each PES packet header.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub pesheader: bool,
    /// Write the DVB subtitle debug traces to console.
    #[arg(long, help_heading = "Heading")]
    pub debugdvdsub: bool,
    /// Ignore PTS jumps (default).
    #[arg(long, help_heading = "Heading")]
    pub ignoreptsjumps: bool,
    /// fix pts jumps. Use this parameter if you
    /// experience timeline resets/jumps in the output.
    #[arg(
        long,
        verbatim_doc_comment,
        conflicts_with = "ignoreptsjumps",
        help_heading = "Heading"
    )]
    pub fixptsjumps: bool,
    /// Reads input from stdin (console) instead of file.
    /// Alternatively, - can be used instead of --stdin
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub stdin: bool,
    #[arg(long, value_name = "x", help_heading = "Heading")]
    pub outinterval: Option<i32>,
    /// When segmenting files, do it only after a I frame
    /// trying to behave like FFmpeg
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub segmentonkeyonly: bool,
    /// Read the input via UDP (listening in the specified port)
    /// instead of reading a file.
    /// Host can be a
    /// hostname or IPv4 address. If host is not specified
    /// then listens on the local host.
    #[arg(
        long,
        value_name = "[host:]port",
        verbatim_doc_comment,
        help_heading = "Heading"
    )]
    pub udp: Option<String>,
    /// Can be a hostname or IPv4 address.
    #[arg(
        long,
        value_name = "port",
        verbatim_doc_comment,
        help_heading = "Heading"
    )]
    pub src: Option<String>,
    /// Sends data in BIN format to the server
    /// according to the CCExtractor's protocol over
    /// TCP. For IPv6 use [address] instead
    #[arg(
        long,
        value_name = "port",
        verbatim_doc_comment,
        help_heading = "Heading"
    )]
    pub sendto: Option<String>,
    /// Specfies optional port for sendto
    #[arg(
        long,
        value_name = "port",
        verbatim_doc_comment,
        help_heading = "Heading"
    )]
    pub sendto_port: Option<u16>,
    /// Reads the input da`ta in BIN format according to
    /// CCExtractor's protocol, listening specified port on the
    /// local host
    #[arg(
        long,
        value_name = "port",
        verbatim_doc_comment,
        help_heading = "Heading"
    )]
    pub tcp: Option<u16>,
    /// Sets server password for new connections to
    /// tcp server
    #[arg(
        long,
        value_name = "port",
        verbatim_doc_comment,
        help_heading = "Heading"
    )]
    pub tcp_password: Option<String>,
    /// Sends to the server short description about
    /// captions e.g. channel name or file name
    #[arg(
        long,
        value_name = "port",
        verbatim_doc_comment,
        help_heading = "Heading"
    )]
    pub tcp_description: Option<String>,
    /// Output field1 data, field2 data, or both
    #[arg(
        long,
        value_name = "1/2/both",
        verbatim_doc_comment,
        help_heading = "Heading"
    )]
    pub output_field: Option<String>,
    /// Use --append to prevent overwriting of existing files. The output will be
    /// appended instead.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub append: bool,
    /// When in srt/sami mode, process captions in channel 2
    /// instead of channel 1.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub cc2: bool,
    /// Enable CEA-708 (DTVCC) captions processing for the listed
    /// services. The parameter is a comma delimited list
    /// of services numbers, such as "1,2" to process the
    /// primary and secondary language services.
    /// Pass "all" to process all services found.
    /// If captions in a service are stored in 16-bit encoding,
    /// you can specify what charset or encoding was used. Pass
    /// its name after service number (e.g. "1[EUC-KR],3" or
    /// "all[EUC-KR]") and it will encode specified charset to
    /// UTF-8 using iconv. See iconv documentation to check if
    /// required encoding/charset is supported.
    #[arg(
        long = "service",
        value_name = "services",
        verbatim_doc_comment,
        help_heading = "Heading"
    )]
    pub cea708services: Option<String>,
    /// With the exception of McPoodle's raw format, which is just the closed
    /// caption data with no other info, CCExtractor can usually detect the
    /// input format correctly. Use this parameter to override the detected
    #[arg(
        long,
        value_name = "format",
        verbatim_doc_comment,
        help_heading = "Heading"
    )]
    pub input: Option<String>,
    #[arg(long, hide = true)]
    pub es: bool,
    #[arg(long, hide = true)]
    pub ts: bool,
    #[arg(long, hide = true)]
    pub ps: bool,
    #[arg(long, hide = true)]
    pub asf: bool,
    #[arg(long, hide = true)]
    pub wtv: bool,
    #[arg(long, hide = true)]
    pub mp4: bool,
    #[arg(long, hide = true)]
    pub mkv: bool,
    #[arg(long, hide = true)]
    pub dvr_ms: bool,
    #[arg(long, value_name = "format", help_heading = "Heading")]
    pub out: Option<String>,
    #[arg(long, hide = true)]
    pub srt: bool,
    #[arg(long, hide = true)]
    pub webvtt: bool,
    #[arg(long, hide = true)]
    pub sami: bool,
    #[arg(long, hide = true)]
    pub smi: bool,
    #[arg(long, hide = true)]
    pub dvdraw: bool,
    #[arg(long, hide = true)]
    pub mcc: bool,
    #[arg(long, hide = true)]
    pub txt: bool,
    #[arg(long, hide = true)]
    pub ttxt: bool,
    #[arg(long, hide = true)]
    pub null: bool,
    /// Use GOP for timing instead of PTS. This only applies
    /// to Program or Transport Streams with MPEG2 data and
    /// overrides the default PTS timing.
    /// GOP timing is always used for Elementary Streams.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub goptime: bool,
    /// Never use GOP timing (use PTS), even if ccextractor
    /// detects GOP timing is the reasonable choice.
    #[arg(
        long,
        verbatim_doc_comment,
        conflicts_with = "goptime",
        help_heading = "Heading"
    )]
    pub no_goptime: bool,
    /// Fix padding - some cards (or providers, or whatever)
    /// seem to send 0000 as CC padding instead of 8080. If you
    /// get bad timing, this might solve it.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub fixpadding: bool,
    /// Use 90090 (instead of 90000) as MPEG clock frequency.
    /// (reported to be needed at least by Panasonic DMR-ES15
    /// DVD Recorder)
    #[arg(long = "90090", verbatim_doc_comment, help_heading = "Heading")]
    pub mpeg90090: bool,
    /// By default, ccextractor will process input files in
    /// sequence as if they were all one large file (i.e.
    /// split by a generic, non video-aware tool. If you
    /// are processing video hat was split with a editing
    /// tool, use --videoedited so ccextractor doesn't try to rebuild
    /// the original timing.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub videoedited: bool,
    /// Consider the file as a continuous stream that is
    /// growing as ccextractor processes it, so don't try
    /// to figure out its size and don't terminate processing
    /// when reaching the current end (i.e. wait for more
    /// data to arrive). If the optional parameter secs is
    /// present, it means the number of seconds without any
    /// new data after which ccextractor should exit. Use
    /// this parameter if you want to process a live stream
    /// but not kill ccextractor externally.
    /// Note: If --s is used then only one input file is
    /// allowed.
    #[arg(short, long, verbatim_doc_comment, help_heading = "Heading")]
    pub stream: Option<String>,
    /// Use the pic_order_cnt_lsb in AVC/H.264 data streams
    /// to order the CC information.  The default way is to
    /// use the PTS information.  Use this switch only when
    /// needed.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub usepicorder: bool,
    /// Force MythTV code branch.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub myth: bool,
    /// Disable MythTV code branch.
    /// The MythTV branch is needed for analog captures where
    /// the closed caption data is stored in the VBI, such as
    /// those with bttv cards (Hauppage 250 for example). This
    /// is detected automatically so you don't need to worry
    /// about this unless autodetection doesn't work for you.
    #[arg(
        long,
        verbatim_doc_comment,
        conflicts_with = "myth",
        help_heading = "Heading"
    )]
    pub no_myth: bool,
    /// This switch works around a bug in Windows 7's built in
    /// software to convert *.wtv to *.dvr-ms. For analog NTSC
    /// recordings the CC information is marked as digital
    /// captions. Use this switch only when needed.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub wtvconvertfix: bool,
    /// Read the captions from the MPEG2 video stream rather
    /// than the captions stream in WTV files
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub wtvmpeg2: bool,
    /// In TS mode, specifically select a program to process.
    /// Not needed if the TS only has one. If this parameter
    /// is not specified and CCExtractor detects more than one
    /// program in the input, it will list the programs found
    /// and terminate without doing anything, unless
    /// --autoprogram (see below) is used.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub program_number: Option<String>,
    /// If there's more than one program in the stream, just use
    /// the first one we find that contains a suitable stream.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub autoprogram: bool,
    /// Uses multiple programs from the same input stream.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub multiprogram: bool,
    /// Don't try to find out the stream for caption/teletext
    /// data, just use this one instead.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub datapid: Option<String>,
    /// Instead of selecting the stream by its PID, select it
    /// by its type (pick the stream that has this type in
    /// the PMT)
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub datastreamtype: Option<String>,
    /// Assume the data is of this type, don't autodetect. This
    /// parameter may be needed if --datapid or --datastreamtype
    /// is used and CCExtractor cannot determine how to process
    /// the stream. The value will usually be 2 (MPEG video) or
    /// 6 (MPEG private data).
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub streamtype: Option<String>,
    /// If the video was recorder using a Hauppauge card, it
    /// might need special processing. This parameter will
    /// force the special treatment.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub hauppauge: bool,
    /// In MP4 files the closed caption data can be embedded in
    /// the video track or in a dedicated CC track. If a
    /// dedicated track is detected it will be processed instead
    /// of the video track. If you need to force the video track
    /// to be processed instead use this option.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub mp4vidtrack: bool,
    /// Some streams come with broadcast date information. When
    /// such data is available, CCExtractor will set its time
    /// reference to the received data. Use this parameter if
    /// you prefer your own reference. Note: Current this only
    /// affects Teletext in timed transcript with --datets.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub no_autotimeref: bool,
    /// Ignore SCTE-20 data if present.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub no_scte20: bool,
    /// Create a separate file for CSS instead of inline.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub webvtt_create_css: bool,
    /// Enable debug so the calculated distance for each two
    /// strings is displayed. The output includes both strings,
    /// the calculated distance, the maximum allowed distance,
    /// and whether the strings are ultimately considered  
    /// equivalent or not, i.e. the calculated distance is
    /// less or equal than the max allowed.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub deblev: bool,
    /// Analyze the video stream even if it's not used for
    /// subtitles. This allows to provide video information.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub analyzevideo: bool,
    /// Enable the X-TIMESTAMP-MAP header for WebVTT (HLS)
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub timestamp_map: bool,
    // /// Don't attempt to correct typos with Levenshtein distance.
    // #[arg(long, verbatim_doc_comment, help_heading="Heading")]
    // pub no_levdist: bool,
    /// Minimum distance we always allow regardless
    /// of the length of the strings.Default 2.
    /// This means that if the calculated distance
    /// is 0,1 or 2, we consider the strings to be equivalent.
    #[arg(
        long,
        value_name = "value",
        verbatim_doc_comment,
        help_heading = "Heading"
    )]
    pub levdistmincnt: Option<String>,
    /// Maximum distance we allow, as a percentage of
    /// the shortest string length. Default 10%.0
    /// For example, consider a comparison of one string of
    ///      30 characters and one of 60 characters. We want to
    /// determine whether the first 30 characters of the longer
    /// string are more or less the same as the shortest string,
    ///      i.e. whether the longest string  is the shortest one
    /// plus new characters and maybe some corrections. Since
    /// the shortest string is 30 characters and  the default
    /// percentage is 10%, we would allow a distance of up
    /// to 3 between the first 30 characters.
    #[arg(
        long,
        value_name = "value",
        verbatim_doc_comment,
        help_heading = "Heading"
    )]
    pub levdistmaxpct: Option<String>,
    /// (Experimental) Produces a chapter file from MP4 files.
    /// Note that this must only be used with MP4 files,
    /// for other files it will simply generate subtitles file.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub chapters: bool,
    /// Append a BOM (Byte Order Mark) to output files.
    /// Note that most text processing tools in linux will not
    /// like BOM.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub bom: bool,
    /// Do not append a BOM (Byte Order Mark) to output
    /// files. Note that this may break files when using
    /// Windows. This is the default in non-Windows builds.
    #[arg(
        long,
        verbatim_doc_comment,
        conflicts_with = "bom",
        help_heading = "Heading"
    )]
    pub no_bom: bool,
    /// Encode subtitles in Unicode instead of Latin-1.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub unicode: bool,
    /// Encode subtitles in UTF-8 (no longer needed.
    /// because UTF-8 is now the default).
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub utf8: bool,
    /// Encode subtitles in Latin-1
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub latin1: bool,
    /// For .srt/.sami/.vtt, don't add font color tags.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub no_fontcolor: bool,
    /// For .srt/.sami/.vtt, don't covert html unsafe character
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub no_htmlescape: bool,
    /// For .srt/.sami/.vtt, don't add typesetting tags.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub no_typesetting: bool,
    /// Trim lines.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub trim: bool,
    /// Select a different default color (instead of
    /// white). This causes all output in .srt/.smi/.vtt
    /// files to have a font tag, which makes the files
    /// larger. Add the color you want in RGB, such as
    /// --dc #FF0000 for red.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub defaultcolor: Option<String>,
    /// Sentence capitalization. Use if you hate
    /// ALL CAPS in subtitles.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub sentencecap: bool,
    /// Add the contents of 'file' to the list of words
    /// that must be capitalized. For example, if file
    /// is a plain text file that contains
    ///
    /// Tony
    /// Alan
    ///
    /// Whenever those words are found they will be written
    /// exactly as they appear in the file.
    /// Use one line per word. Lines starting with # are
    /// considered comments and discarded.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub capfile: Option<String>,
    /// Censors profane words from subtitles.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub kf: bool,
    /// Add the contents of <file> to the list of words that.
    /// must be censored. The content of <file>, follows the
    /// same syntax as for the capitalization file
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "file",
        help_heading = "Heading"
    )]
    pub profanity_file: Option<String>,
    /// Split output text so each frame contains a complete
    /// sentence. Timings are adjusted based on number of
    /// characters
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub splitbysentence: bool,
    /// For timed transcripts that have an absolute date
    /// instead of a timestamp relative to the file start), use
    /// this time reference (UNIX timestamp). 0 => Use current
    /// system time.
    /// ccextractor will automatically switch to transport
    /// stream UTC timestamps when available.
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "REF",
        help_heading = "Heading"
    )]
    pub unixts: Option<String>,
    /// In transcripts, write time as YYYYMMDDHHMMss,ms.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub datets: bool,
    /// In transcripts, write time as ss,ms
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub sects: bool,
    /// Transcripts are generated with a specific format
    /// that is convenient for a specific project, feel
    /// free to play with it but be aware that this format
    /// is really live - don't rely on its output format
    /// not changing between versions.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub ucla: bool,
    /// Map Latin symbols to Cyrillic ones in special cases
    /// of Russian Teletext files (issue #1086)
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub latrusmap: bool,
    /// In timed transcripts, all XDS information will be saved
    /// to the output file.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub xds: bool,
    /// Use LF (UNIX) instead of CRLF (DOS, Windows) as line
    /// terminator.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub lf: bool,
    /// For MCC Files, force dropframe frame count.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub df: bool,
    /// Based on position on screen, attempt to determine
    /// the different speakers and a dash (-) when each
    /// of them talks (.srt/.vtt only, --trim required).
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub autodash: bool,
    /// produce an XMLTV file containing the EPG data from
    /// the source TS file. Mode: 1 = full output
    /// 2 = live output. 3 = both
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "mode",
        help_heading = "Heading"
    )]
    pub xmltv: Option<String>,
    /// interval of x seconds between writing live mode xmltv output.
    #[arg(long, verbatim_doc_comment, value_name = "x", help_heading = "Heading")]
    pub xmltvliveinterval: Option<String>,
    /// interval of x seconds between writing full file xmltv output.
    #[arg(long, verbatim_doc_comment, value_name = "x", help_heading = "Heading")]
    pub xmltvoutputinterval: Option<String>,
    /// Only print current events for xmltv output.
    #[arg(long, verbatim_doc_comment, value_name = "x", help_heading = "Heading")]
    pub xmltvonlycurrent: Option<String>,
    /// Create a .sem file for each output file that is open
    /// and delete it on file close.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub sem: bool,
    /// For DVB subtitles, select which language's caption
    /// stream will be processed. e.g. 'eng' for English.
    /// If there are multiple languages, only this specified
    /// language stream will be processed (default).
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub dvblang: Option<String>,
    /// Manually select the name of the Tesseract .traineddata
    /// file. Helpful if you want to OCR a caption stream of
    /// one language with the data of another language.
    /// e.g. '-dvblang chs --ocrlang chi_tra' will decode the
    /// Chinese (Simplified) caption stream but perform OCR
    /// using the Chinese (Traditional) trained data
    /// This option is also helpful when the traineddata file
    /// has non standard names that don't follow ISO specs
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub ocrlang: Option<String>,
    /// How to quantize the bitmap before passing it to tesseract
    /// for OCR'ing.
    /// 0: Don't quantize at all.
    /// 1: Use CCExtractor's internal function (default).
    /// 2: Reduce distinct color count in image for faster results.
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "mode",
        help_heading = "Heading"
    )]
    pub quant: Option<i32>,
    /// Select the OEM mode for Tesseract.
    /// Available modes :
    /// 0: OEM_TESSERACT_ONLY - the fastest mode.
    /// 1: OEM_LSTM_ONLY - use LSTM algorithm for recognition.
    /// 2: OEM_TESSERACT_LSTM_COMBINED - both algorithms.
    /// Default value depends on the tesseract version linked :
    /// Tesseract v3 : default mode is 0,
    /// Tesseract v4 : default mode is 1.
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "mode",
        help_heading = "Heading"
    )]
    pub oem: Option<i32>,
    /// For MKV subtitles, select which language's caption
    /// stream will be processed. e.g. 'eng' for English.
    /// Language codes can be either the 3 letters bibliographic
    /// ISO-639-2 form (like "fre" for french) or a language
    /// code followed by a dash and a country code for specialities
    /// in languages (like "fre-ca" for Canadian French).
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "lang",
        help_heading = "Heading"
    )]
    pub mkvlang: Option<String>,
    /// When processing DVB don't use the OCR to write the text as
    /// comments in the XML file.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub no_spupngocr: bool,
    /// Specify the full path of the font that is to be used when
    /// generating SPUPNG files. If not specified, you need to
    /// have the default font installed (Helvetica for macOS, Calibri
    /// for Windows, and Noto for other operating systems at their
    /// default location)
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "path",
        help_heading = "Heading"
    )]
    pub font: Option<String>,
    /// Specify the full path of the italics font that is to be used when
    /// generating SPUPNG files. If not specified, you need to
    /// have the default font installed (Helvetica Oblique for macOS, Calibri Italic
    /// for Windows, and NotoSans Italic for other operating systems at their
    /// default location)
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "path",
        help_heading = "Heading"
    )]
    pub italics: Option<String>,
    /// Forces input buffering.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub bufferinput: bool,
    /// Disables input buffering.
    #[arg(
        long,
        verbatim_doc_comment,
        conflicts_with = "bufferinput",
        help_heading = "Heading"
    )]
    pub no_bufferinput: bool,
    /// Specify a size for reading, in bytes (suffix with K or
    /// or M for kilobytes and megabytes). Default is 16M.
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "val",
        help_heading = "Heading"
    )]
    pub buffersize: Option<String>,
    /// keep-output-close. If used then CCExtractor will close
    /// the output file after writing each subtitle frame and
    /// attempt to create it again when needed.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub koc: bool,
    /// Flush the file buffer whenever content is written.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub forceflush: bool,
    /// Direct Roll-Up. When in roll-up mode, write character by
    /// character instead of line by line. Note that this
    /// produces (much) larger files.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub dru: bool,
    /// If you hate the repeated lines caused by the roll-up
    /// emulation, you can have ccextractor write only one
    /// line at a time, getting rid of these repeated lines.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub no_rollup: bool,
    /// roll-up captions can consist of 2, 3 or 4 visible
    /// lines at any time (the number of lines is part of
    /// the transmission). If having 3 or 4 lines annoys
    /// you you can use --ru to force the decoder to always
    /// use 1, 2 or 3 lines. Note that 1 line is not
    /// a real mode rollup mode, so CCExtractor does what
    /// it can.
    /// In --ru1 the start timestamp is actually the timestamp
    /// of the first character received which is possibly more
    /// accurate.
    #[arg(
        long = "ru1",
        conflicts_with = "rollup2",
        verbatim_doc_comment,
        value_name = "type",
        help_heading = "Heading"
    )]
    pub rollup1: bool,
    #[arg(
        long = "ru2",
        conflicts_with = "rollup3",
        verbatim_doc_comment,
        value_name = "type",
        help_heading = "Heading"
    )]
    pub rollup2: bool,
    #[arg(
        long = "ru3",
        conflicts_with = "rollup1",
        verbatim_doc_comment,
        value_name = "type",
        help_heading = "Heading"
    )]
    pub rollup3: bool,
    /// For srt/sami/webvtt, add this number of milliseconds to
    /// all times. For example, --delay 400 makes subtitles
    /// appear 400ms late. You can also use negative numbers
    /// to make subs appear early.
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "ms",
        help_heading = "Heading"
    )]
    pub delay: Option<i64>,
    /// Only write caption information that starts after the
    /// given time.
    /// Time can be seconds, MM:SS or HH:MM:SS.
    /// For example, --startat 3:00 means 'start writing from
    /// minute 3.
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "time",
        help_heading = "Heading"
    )]
    pub startat: Option<String>,
    /// Stop processing after the given time (same format as
    /// --startat).
    /// The --startat and --endat options are honored in all
    /// output formats.  In all formats with timing information
    /// the times are unchanged.
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "time",
        help_heading = "Heading"
    )]
    pub endat: Option<String>,
    /// Write 'num' screenfuls and terminate processing.
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "num",
        help_heading = "Heading"
    )]
    pub screenfuls: Option<String>,
    /// --codec dvbsub
    ///     select the dvb subtitle from all elementary stream,
    ///     if stream of dvb subtitle type is not found then
    ///     nothing is selected and no subtitle is generated
    /// --codec teletext
    ///     select the teletext subtitle from elementary stream
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "value",
        help_heading = "Heading"
    )]
    pub codec: Option<String>,
    /// --no-codec dvbsub
    ///     ignore dvb subtitle and follow default behaviour
    /// --no-codec teletext
    ///     ignore teletext subtitle
    #[arg(
        long,
        verbatim_doc_comment,
        conflicts_with = "codec",
        value_name = "value",
        help_heading = "Heading"
    )]
    pub no_codec: Option<String>,
    /// Write this text as start credits. If there are
    /// several lines, separate them with the
    /// characters \n, for example Line1\nLine 2.
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "text",
        help_heading = "Heading"
    )]
    pub startcreditstext: Option<String>,
    /// Don't display the start credits before this
    /// time (S, or MM:SS). Default: 0
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "time",
        help_heading = "Heading"
    )]
    pub startcreditsnotbefore: Option<String>,
    /// Don't display the start credits after this
    /// time (S, or MM:SS). Default: 5:00
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "time",
        help_heading = "Heading"
    )]
    pub startcreditsnotafter: Option<String>,
    /// Start credits need to be displayed for at least
    /// this time (S, or MM:SS). Default: 2
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "time",
        help_heading = "Heading"
    )]
    pub startcreditsforatleast: Option<String>,
    /// Start credits should be displayed for at most
    /// this time (S, or MM:SS). Default: 5
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "time",
        help_heading = "Heading"
    )]
    pub startcreditsforatmost: Option<String>,
    /// Write this text as end credits. If there are
    /// several lines, separate them with the
    /// characters \n, for example Line1\nLine 2.
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "txt",
        help_heading = "Heading"
    )]
    pub endcreditstext: Option<String>,
    /// End credits need to be displayed for at least
    /// this time (S, or MM:SS). Default: 2
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "time",
        help_heading = "Heading"
    )]
    pub endcreditsforatleast: Option<String>,
    /// End credits should be displayed for at most
    /// this time (S, or MM:SS). Default: 5
    #[arg(
        long,
        verbatim_doc_comment,
        value_name = "time",
        help_heading = "Heading"
    )]
    pub endcreditsforatmost: Option<String>,
    /// Show lots of debugging output.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub debug: bool,
    /// Print debug traces from the EIA-608 decoder.
    /// If you need to submit a bug report, please send
    /// the output from this option.
    #[arg(long = "608", verbatim_doc_comment, help_heading = "Heading")]
    pub eia608: bool,
    /// Print debug information from the (currently
    /// in development) EIA-708 (DTV) decoder.
    #[arg(long = "708", verbatim_doc_comment, help_heading = "Heading")]
    pub eia708: bool,
    /// Enable lots of time stamp output.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub goppts: bool,
    /// Enable XDS debug data (lots of it).
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub xdsdebug: bool,
    /// Print debug info about the analysed elementary
    /// video stream.
    #[arg(long, verbatim_doc_comment, help_heading = "Heading")]
    pub vides: bool,
}
