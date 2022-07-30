import ./common

const IDEA_LABEL = "__test_idea_10105"

let createdIdea = block:
  let resp = createIdea IDEA_LABEL
  doAssert resp.status == Http200
  let createdIdea = resp.asJson.asIdea
  doAssert createdIdea.label == IDEA_LABEL
  createdIdea

block:
  let resp = getIdea createdIdea.id
  doAssert resp.status == Http200
  doAssert resp.asJson.asIdea == createdIdea

let numIdeas = block:
  let resp = getNumIdeas()
  doAssert resp.status == Http200
  doAssert not(resp.asInt == 0)
  resp.asInt

block:
  let
    pageSize = 10
    pageNum = ceilDiv(numIdeas, pageSize) - 1
    resp = getIdeas(pageNum, pageSize)
  doAssert resp.status == Http200
  doAssert resp.asJson.asGetideasResp.ideas[^1] == createdIdea

block:
  let resp = deleteIdea createdIdea.id
  doAssert resp.status == Http200
  doAssert resp.asInt == 1

block:
  let resp = getIdea createdIdea.id
  doAssert not(resp.status == Http200)

