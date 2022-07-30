import ./common

const DOCUMENT_TITLE = "Anas's Grimoire of Red Magic"

let createdDocument = block:
  let resp = createDocument "ogn-test/Anas's Grimoire of Red Magic.pdf"
  doAssert resp.status == Http200

  let
    createdDocuments = resp.asJson.asUploadDocumentsResp
    createdDocument = createdDocuments[^1]

  doAssert createdDocument.title == DOCUMENT_TITLE
  createdDocument

block:
  let resp = getDocument createdDocument.id
  doAssert resp.status == Http200
  doAssert resp.asJson.asDocument == createdDocument

let numDocuments = block:
  let resp = getNumDocuments()
  doAssert resp.status == Http200
  doAssert not(resp.asInt == 0)
  resp.asInt

block:
  let
    pageSize = 10
    pageNum = ceilDiv(numDocuments, pageSize) - 1
    resp = getDocuments(pageNum, pageSize)
  doAssert resp.status == Http200
  doAssert resp.asJson.asGetDocumentsResp.documents[^1] == createdDocument

block:
  let resp = deleteDocument createdDocument.id
  doAssert resp.status == Http200
  doAssert resp.asInt == 1

block:
  let resp = getDocument createdDocument.id
  doAssert not(resp.status == Http200)
