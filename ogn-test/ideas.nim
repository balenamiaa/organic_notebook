import ./common

const IDEA_LABEL = "__test_idea_10105"

let createResp = makeReq("/ideas", "post", "status,json") do:
  $ %*{"label": IDEA_LABEL}
do:
  {"content-type": "application/json"}.newHttpHeaders()

doAssert createResp.status == Http200
doAssert createResp.json["label"].str == IDEA_LABEL

let getResp = makeReq("/ideas/" & createResp.json["id"].str, "get", "status,json")

doAssert getResp.status == Http200
doAssert getResp.json["label"].str == IDEA_LABEL

let getNumResp = makeReq("/ideas//num", "get", "status,string")

doAssert getNumResp.status == Http200
doAssert not(getNumResp.val.parseInt == 0)

let
  pageSize = 10
  pageNum = ceilDiv(getNumResp.val.parseInt, pageSize) - 1

let ideasResp = makeReq("/ideas?page_num=" & $pageNum & "&page_size=" &
    $pageSize, "get", "status,json")

doAssert ideasResp.status == Http200
doAssert ideasResp.json["ideas"].items.toSeq[^1]["label"].str == IDEA_LABEL

let deleteIdeaResp = makeReq("/ideas/" & createResp.json["id"].str, "delete", "status,string")

doAssert deleteIdeaResp.status == Http200
doAssert deleteIdeaResp.val.parseInt == 1 # num of deleted ideas

let attemptGetResp = makeReq("/ideas/" & createResp.json["id"].str, "get",
    "status") # attempt to get deleted idea

doAssert not(attemptGetResp == Http200)

