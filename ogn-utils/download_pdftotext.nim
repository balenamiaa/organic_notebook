import std/[httpclient, os]

const
  WINDOWS_URL = r"http://dl.xpdfreader.com/xpdf-tools-win-4.04.zip"
  LINUX_URL = r"http://dl.xpdfreader.com/xpdf-tools-linux-4.04.tar.gz"


proc downloadArchive(): string =
  let
    dir = getTempDir()
    url = when defined windows: WINDOWS_URL else: LINUX_URL
    path = when defined windows:
      dir / "xpdf-tools-win-4.04.zip"
    else:
      dir / "xpdf-tools-linux-4.04.tar.gz"
  newHttpClient().downloadFile(url, path)
  path


let 
  outputDir = getHomeDir() / ".ogn-tools"
  arch = when defined x86: "bin32" else: "bin64"
  archivePath = downloadArchive()
  

createDir outputDir

when defined windows:
  # windows tar doesn't support overwriting. will delete in nim code
  removeDir outputDir / "xpdf-tools-win-4.04"

  let status = execShellCmd "tar -zxf " & archivePath & " -C " & outputDir
  assert status == 0
  echo outputDir / "xpdf-tools-win-4.04" / arch / "pdftotext.exe"
else:
  let status = execShellCmd "tar --overwrite -zxf " & archivePath & " -C " & outputDir
  assert status == 0
  echo outputDir / "xpdf-tools-linux-4.04" / arch / "pdftotext"
