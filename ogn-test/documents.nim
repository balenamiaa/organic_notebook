import ./common

const DOCUMENT_TITLE = "Anas's Grimoire of Red Magic"

let createResp = makeReq("/documents", "post", "status,json", files = block:
  var files = newMultipartData()
  files.addFiles({"test_document": "./Anas's Grimoire of Red Magic.pdf"})
)

doAssert createResp.status == Http200

let createdDocument = createResp.json.items.toSeq()[^1];

doAssert createdDocument["title"].str == DOCUMENT_TITLE

let getResp = makeReq("/documents/" & createdDocument["id"].str, "get", "status,json")

doAssert getResp.status == Http200
doAssert getResp.json["title"].str == DOCUMENT_TITLE

let getNumResp = makeReq("/documents//num", "get", "status,string")

doAssert getNumResp.status == Http200
doAssert not(getNumResp.val.parseInt == 0)

let
  pageSize = 10
  pageNum = ceilDiv(getNumResp.val.parseInt, pageSize) - 1

let documentsResp = makeReq("/documents?page_num=" & $pageNum & "&page_size=" &
    $pageSize, "get", "status,json")

doAssert documentsResp.status == Http200
doAssert documentsResp.json["documents"].items.toSeq[^1]["title"].str == DOCUMENT_TITLE

let deleteIdeaResp = makeReq("/documents/" & createdDocument["id"].str,
    "delete", "status,string")

doAssert deleteIdeaResp.status == Http200
doAssert deleteIdeaResp.val.parseInt == 1 # num of deleted ideas

let attemptGetResp = makeReq("/ideas/" & createdDocument["id"].str, "get",
    "status") # attempt to get deleted idea

doAssert not(attemptGetResp == Http200)
