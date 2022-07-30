import ./common



const DOCUMENT_TITLE = "Anas's Grimoire of Red Magic"
const IDEA_LABEL = "__test_idea_10105"
const IDEA_REF_TEXT = "test idea ref 0"

let createdDocument = block:
  let resp = createDocument "ogn-test/Anas's Grimoire of Red Magic.pdf"
  doAssert resp.status == Http200

  let
    createdDocuments = resp.asJson.asUploadDocumentsResp
    createdDocument = createdDocuments[^1]
  createdDocument

let createdIdea = block:
  let resp = createIdea IDEA_LABEL
  doAssert resp.status == Http200
  resp.asJson.asIdea


let createdIdeaRef = block:
  let resp = createIdeaRef(
    docPage = DocumentPage(
      docId: createdDocument.id,
      pageNum: 6
    ),
    idea = createdIdea,
    text = IDEA_REF_TEXT
  )
  doAssert resp.status == Http200

  let createdIdeaRef = resp.asJson.asIdeaRef

  doAssert createdIdeaRef.refId == createdIdea.id
  doAssert createdIdeaRef.ideaRefText == IDEA_REF_TEXT
  doAssert createdIdeaRef.docPage == DocumentPage(
    docId: createdDocument.id,
    pageNum: 6
  )
  createdIdeaRef

block:
  let resp = getIdeaRef createdIdeaRef.id
  doAssert resp.status == Http200
  doAssert resp.asJson.asIdeaRef == createdIdeaRef

let numIdeaRefs = block:
  let resp = getNumIdeaRefs()
  doAssert resp.status == Http200
  doAssert not(resp.asInt() == 0)
  resp.asInt()

block:
  let
    pageSize = 10
    pageNum = ceilDiv(numIdeaRefs, pageSize) - 1
    resp = getIdeaRefs(pageNum, pageSize)
  doAssert resp.status == Http200
  doAssert resp.asJson.asGetIdeaRefsResp.ideaRefs[^1] == createdIdeaRef

let numIdeaRefsForIdea = block:
  let resp = getNumIdeaRefsForIdea createdIdea
  doAssert resp.status == Http200
  resp.asInt()

block:
  let
    pageSize = 10
    pageNum = ceilDiv(numIdeaRefsForIdea, pageSize) - 1
    resp = getIdeaRefsForIdea(createdIdea, pageNum, pageSize)
  doAssert resp.status == Http200
  let ideaRefs = resp.asJson.asGetIdeaRefsForIdeaResp.ideaRefs
  doAssert ideaRefs[^1] == createdIdeaRef

block:
  let resp = deleteIdeaRef createdIdeaRef.id
  doAssert resp.status == Http200
  doAssert resp.asInt() == 1

block:
  let resp = getIdeaRef createdIdeaRef.id
  doAssert not(resp.status == Http200)

block:
  let resp = deleteIdea createdIdea.id
  doAssert resp.status == Http200

block:
  let resp = deleteDocument createdDocument.id
  doAssert resp.status == Http200
