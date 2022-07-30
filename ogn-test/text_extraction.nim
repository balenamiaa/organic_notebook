import ./common


const
  DOCUMENT_TITLE = "Anas's Grimoire of Red Magic"
  DOCUMENT_LASTPAGE_CONTENT_WINDOWS = "CONCLUSION\r\n\r\n� IRON DEFICIENCY IS COMMONEST TYPE OF ANEMIA\r\n� BLOOD LOSS MUST BE EXCLUDED\r\n� ORAL IRON IS BEST therapy."
  DOCUMENT_LASTPAGE_CONTENT_LINUX = "CONCLUSION\n\n� IRON DEFICIENCY IS COMMONEST TYPE OF ANEMIA\n� BLOOD LOSS MUST BE EXCLUDED\n� ORAL IRON IS BEST therapy."
  DOCUMENT_LASTPAGE_CONTENT = when defined windows: DOCUMENT_LASTPAGE_CONTENT_WINDOWS else: DOCUMENT_LASTPAGE_CONTENT_LINUX
let createdDocument = block:
  let resp = createDocument "ogn-test/Anas's Grimoire of Red Magic.pdf"
  doAssert resp.status == Http200

  let
    createdDocuments = resp.asJson.asUploadDocumentsResp
    createdDocument = createdDocuments[^1]

  doAssert createdDocument.title == DOCUMENT_TITLE
  createdDocument

let extractedTexts = block:
  let resp = extractText createdDocument
  doAssert resp.status == Http200
  let extractedTexts = resp.asJson.asExtractTextResp
  doAssert extractedTexts[^1].content == DOCUMENT_LASTPAGE_CONTENT
  extractedTexts

block:
  let resp = getExtractedTextsForDoc createdDocument
  doAssert resp.status == Http200
  let extractedTextsNew = resp.asJson.asGetExtractedTextsForDocResp
  doAssert extractedTextsNew == extractedTexts

block:
  let resp = getExtractedTextsForDocBulk [createdDocument]
  doAssert resp.status == Http200
  let extractedTextsNew = resp.asJson.asGetExtractedTextsForDocBulkResp
  doAssert extractedTextsNew == extractedTexts

block:
  let resp = getNumExtractedTextsForDoc createdDocument
  doAssert resp.status == Http200
  doAssert resp.asInt == extractedTexts.len

let numExtractedTexts = block:
  let resp = getNumExtractedTexts()
  doAssert resp.status == Http200
  doAssert resp.asInt == extractedTexts.len
  resp.asInt

block:
  let
    pageSize = 10
    pageNum = ceilDiv(numExtractedTexts, pageSize) - 1
    resp = getExtractedTexts(pageNum, pageSize)
  doAssert resp.status == Http200
  doAssert resp.asJson.asGetExtractedTextsResp.extracted_texts[^1] ==
      extractedTexts[^1]

block:
  let resp = deleteExtractedText createdDocument
  doAssert resp.status == Http200
  doAssert resp.asInt == extractedTexts.len

block:
  let resp = deleteDocument createdDocument.id
  doAssert resp.status == Http200
