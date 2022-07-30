import ./common


const DOCUMENT_TITLE = "Anas's Grimoire of Red Magic"
const DOCUMENT_LASTPAGE_CONTENT = "CONCLUSION\r\n\r\n� IRON DEFICIENCY IS COMMONEST TYPE OF ANEMIA\r\n� BLOOD LOSS MUST BE EXCLUDED\r\n� ORAL IRON IS BEST therapy."

let createdDocument = block:
  let resp = createDocument "ogn-test/Anas's Grimoire of Red Magic.pdf"
  doAssert resp.status == Http200

  let
    createdDocuments = resp.asJson.asUploadDocumentsResp
    createdDocument = createdDocuments[^1]

  doAssert createdDocument.title == DOCUMENT_TITLE
  createdDocument

let extractedText = block:
  let resp = extractText createdDocument
  doAssert resp.status == Http200
  let extractedText = resp.asJson.asExtractTextResp
  doAssert extractedText[^1].content == DOCUMENT_LASTPAGE_CONTENT
  extractedText

block:
  let resp = deleteExtractedText createdDocument
  doAssert resp.status == Http200
  doAssert resp.asInt == extractedText.len

block:
  let resp = deleteDocument createdDocument.id
  doAssert resp.status == Http200
