import ./common

let createIdeaResp = makeReq("/ideas", "post", "status,json") do:
  $ %*{"label": "__test_idea_10105"}
do:
  {"content-type": "application/json"}.newHttpHeaders()

doAssert createIdeaResp.status == Http200


let createDocResp = makeReq("/documents", "post", "status,json", files = block:
  var files = newMultipartData()
  files.addFiles({"test_document": "ogn-test/Anas's Grimoire of Red Magic.pdf"})
)

doAssert createDocResp.status == Http200

let createdDocument = createDocResp.json.items.toSeq()[^1]

let createIdeaRefResp = makeReq("/idea_refs", "post", "status,json") do:
  $ %*
    {
      "doc_page":
      {
        "document_id": createdDocument["id"].str,
        "page_number": 6
      },
      "idea_ref": createIdeaResp.json["id"].str,
      "idea_ref_text": "test idea ref 0"
    }
do:
  {"content-type": "application/json"}.newHttpHeaders()

doAssert createIdeaRefResp.status == Http200
doAssert createIdeaRefResp.json["idea_ref"] == createIdeaResp.json["id"]
doAssert createIdeaRefResp.json["idea_ref_text"].str == "test idea ref 0"
doAssert createIdeaRefResp.json["doc_page"]["document_id"] == createdDocument["id"]
doAssert createIdeaRefResp.json["doc_page"]["page_number"].getInt == 6

let getResp = makeReq("/idea_refs/" & createIdeaRefResp.json["id"].str, "get", "status,json")

doAssert getResp.status == Http200
doAssert getResp == createIdeaRefResp

let getNumResp = makeReq("/idea_refs//num", "get", "status,string")

doAssert getNumResp.status == Http200
doAssert not(getNumResp.val.parseInt == 0)

let
  pageSize = 10
  pageNum = ceilDiv(getNumResp.val.parseInt, pageSize) - 1

let ideaRefsResp = makeReq("/idea_refs?page_num=" & $pageNum & "&page_size=" &
    $pageSize, "get", "status,json")

doAssert ideaRefsResp.status == Http200
doAssert ideaRefsResp.json["idea_refs"].items.toSeq[^1] ==
    createIdeaRefResp.json

let ideaRefForIdea = block:
  let getNumIdeaRefsForIdeaResp = makeReq("/idea_refs_for_idea/" & createIdeaResp.json["id"].str & "/num", "get", "status,string")

  doAssert getNumIdeaRefsForIdeaResp.status == Http200
  doAssert not(getNumIdeaRefsForIdeaResp.val.parseInt == 0)

  let
    pageSize = 10
    pageNum = ceilDiv(getNumResp.val.parseInt, pageSize) - 1

  let getIdeaRefsForIdeaResp = makeReq("/idea_refs_for_idea/" & createIdeaResp.json["id"].str & "?page_num=" & $pageNum & "&page_size=" & $pageSize, "get", "status,json")
  
  doAssert getIdeaRefsForIdeaResp.status == Http200

  getIdeaRefsForIdeaResp.json["idea_refs"].items.toSeq()[^1]



doAssert ideaRefForIdea == createIdeaRefResp.json
doAssert ideaRefForIdea == createIdeaRefResp.json


let deleteIdeaRefResp = makeReq("/idea_refs/" & createIdeaRefResp.json[
    "id"].str, "delete", "status,string")

doAssert deleteIdeaRefResp.status == Http200
doAssert deleteIdeaRefResp.val.parseInt == 1 # num of deleted ideas

let attemptGetResp = makeReq("/idea_refs/" & createIdeaRefResp.json["id"].str,
    "get", "status") # attempt to get deleted idea

doAssert not(attemptGetResp == Http200)

let deleteIdeaResp = makeReq("/ideas/" & createIdeaResp.json["id"].str,
    "delete", "status")
doAssert deleteIdeaResp == Http200

let deleteDocumentResp = makeReq("/documents/" & createdDocument["id"].str,
    "delete", "status")
doAssert deleteDocumentResp == Http200
