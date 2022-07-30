import std/[httpclient, json, strutils, strformat, math, sequtils]

template key(serializationKey: string) {.pragma.}

type
  DocumentId* = distinct int
  IdeaRefId* = distinct int
  IdeaId* = distinct int
  ExtractedTextId* = distinct int

  Document* = object
    id*: DocumentId
    title*: string
    filetype*: string

  DocumentPage* = object
    docId* {.key: "document_id".}: DocumentId
    pageNum* {.key: "page_number".}: int

  Idea* = object
    id*: IdeaId
    label*: string

  IdeaRef* = object
    id*: IdeaRefId
    docPage* {.key: "doc_page".}: DocumentPage
    refId* {.key: "idea_ref".}: IdeaId
    ideaRefText* {.key: "idea_ref_text".}: string

  ExtractedText* = object
    id*: ExtractedTextId
    content*: string
    docPage*: DocumentPage

  Response* = object
    status*: HttpCode
    body*: string

  UploadDocumentResp* = seq[Document]

  GetDocumentsResp* = object
    documents*: seq[Document]
    num* {.key: "num_documents_retrieved".}: int

  GetIdeasResp* = object
    ideas*: seq[Idea]
    num* {.key: "num_ideas_retrieved".}: int

  GetIdeaRefsResp* = object
    ideaRefs*: seq[IdeaRef]
    num* {.key: "num_idea_refs_retrieved".}: int

  ExtractTextResp* = seq[ExtractedText]



proc `$`*(self: DocumentId): string {.borrow.}
proc `$`*(self: IdeaRefId): string {.borrow.}
proc `$`*(self: IdeaId): string {.borrow.}
proc `$`*(self: ExtractedTextId): string {.borrow.}

proc `==`*(a, b: DocumentId): bool {.borrow.}
proc `==`*(a, b: IdeaRefId): bool {.borrow.}
proc `==`*(a, b: IdeaId): bool {.borrow.}
proc `==`*(a, b: ExtractedTextId): bool {.borrow.}

proc asDocument*(json: JsonNode): Document = Document(
  id: json["id"].getStr.parseInt.DocumentId,
  title: json["title"].getStr,
  filetype: json["filetype"].getStr
)

proc asIdea*(json: JsonNode): Idea = Idea(
  id: json["id"].getStr.parseInt.IdeaId,
  label: json["label"].getStr
)

proc asDocPage(json: JsonNode): DocumentPage = DocumentPage(
  docId: json["document_id"].getStr.parseInt.DocumentId,
  pageNum: json["page_number"].getInt
)

proc asIdeaRef*(json: JsonNode): IdeaRef = IdeaRef(
  id: json["id"].getStr.parseInt.IdeaRefId,
  docPage: json["doc_page"].asDocPage,
  refId: json["idea_ref"].getStr.parseInt.IdeaId,
  ideaRefText: json["idea_ref_text"].getStr
)

proc asExtractedText*(json: JsonNode): ExtractedText = ExtractedText(
  id: json["id"].getStr.parseInt.ExtractedTextId,
  content: json["content"].getStr,
  docPage: json["doc_page"].asDocPage
)

proc asUploadDocumentsResp*(json: JsonNode): UploadDocumentResp =
  for item in json.items:
    result.add item.asDocument()

proc asGetDocumentsResp*(json: JsonNode): GetDocumentsResp =
  for item in json["documents"].items:
    result.documents.add item.asDocument
  result.num = json["num_documents_retrieved"].getInt

proc asGetIdeasResp*(json: JsonNode): GetIdeasResp =
  for item in json["ideas"].items:
    result.ideas.add item.asIdea
  result.num = json["num_ideas_retrieved"].getInt

proc asGetIdeaRefsResp*(json: JsonNode): GetIdeaRefsResp =
  for item in json["idea_refs"].items:
    result.ideaRefs.add item.asIdeaRef
  result.num = json["num_idea_refs_retrieved"].getInt

proc asExtractTextResp*(json: JsonNode): ExtractTextResp =
  for item in json.items:
    result.add item.asExtractedText()


proc makeReq*(
  endpoint: string, `method`: static[string],
  reqBody: string = "", headers: HttpHeaders = newHttpHeaders(),
      files: MultipartData = newMultipartData()
): Response =

  const
    address = "http://127.0.0.1:8080"
    `method` = `method`.toLowerAscii()

  var client = newHttpClient()

  const httpMethod: HttpMethod = when `method` == "get":
    HttpGet
  elif `method` == "post":
    HttpPost
  elif `method` == "delete":
    HttpDelete
  else:
    {.error: "Unknown method: " & `method`.}

  when not defined(noMakeReqDump):
    echo "Sending request to: " & address & "/api" & endpoint
    echo "  Method: " & $httpMethod
    echo "  Body: " & reqBody
    echo "  Headers: " & $headers

  let resp = client.request(address & "/api" & endpoint, httpMethod, reqBody,
      headers, files)

  when not defined(noMakeReqDump):
    echo "Got response for " & address & "/api" & endpoint
    echo "  Status: " & $resp.status
    echo "  Body: " & resp.body
    echo "  Headers: " & $resp.headers

  return Response(
    status: resp.code(),
    body: resp.body()
  )


proc asJson*(self: Response): JsonNode = parseJson self.body
proc asStr*(self: Response): string = self.body
proc asInt*(self: Response): int = self.body.parseInt
proc asFloat*(self: Response): float = self.body.parseFloat

proc createDocument*(filePath: string): Response =
  makeReq("/documents", "post", files = block:
    var files = newMultipartData()
    files.addFiles({"test_document": filePath})
  )

proc deleteDocument*(docId: DocumentId): Response =
  makeReq("/documents/" & $docId, "delete")

proc getDocuments*(pageNum, pageSize: int): Response =
  makeReq("/documents?page_num=" & $pageNum & "&page_size=" & $pageSize, "get")

proc getNumDocuments*(): Response =
  makeReq("/documents//num", "get")

proc getDocument*(docId: DocumentId): Response =
  makeReq("/documents/" & $docId, "get")


proc createIdea*(label: string): Response =
  makeReq("/ideas", "post") do:
    $ %*{"label": label}
  do:
    {"content-type": "application/json"}.newHttpHeaders()

proc deleteIdea*(ideaId: IdeaId): Response =
  makeReq("/ideas/" & $ideaId, "delete")

proc getIdeas*(pageNum, pageSize: int): Response =
  makeReq("/ideas?page_num=" & $pageNum & "&page_size=" & $pageSize, "get")

proc getNumIdeas*(): Response =
  makeReq("/ideas//num", "get")

proc getIdea*(ideaId: IdeaId): Response =
  makeReq("/ideas/" & $ideaId, "get")


proc createIdeaRef(documentId: DocumentId, documentPage: int, ideaId: IdeaId,
    text: string): Response =
  makeReq("/idea_refs", "post") do:
    $ %*
      {
        "doc_page":
        {
          "document_id": $documentId,
          "page_number": documentPage,
        },
        "idea_ref": $ideaId,
        "idea_ref_text": text
      }
  do:
    {"content-type": "application/json"}.newHttpHeaders()

proc createIdeaRef*(docPage: DocumentPage, idea: Idea, text: string): Response =
  createIdeaRef(docPage.docId, docPage.pageNum, idea.id, text)

proc deleteIdeaRef*(ideaRefId: IdeaRefId): Response =
  makeReq("/idea_refs/" & $ideaRefId, "delete")

proc getIdeaRefs*(pageNum, pageSize: int): Response =
  makeReq("/idea_refs?page_num=" & $pageNum & "&page_size=" & $pageSize, "get")

proc getNumIdeaRefs*(): Response =
  makeReq("/idea_refs//num", "get")

proc getIdeaRef*(ideaRefId: IdeaRefId): Response =
  makeReq("/idea_refs/" & $ideaRefId, "get")


proc getNumIdeaRefsForIdea*(idea: Idea): Response =
  makeReq("/idea_refs_for_idea/" & $idea.id & "/num", "get")

proc getIdeaRefsForIdea*(idea: Idea, pageNum, pageSize: int): Response =
  makeReq("/idea_refs_for_idea/" & $idea.id & "?page_num=" & $pageNum &
      "&page_size=" & $pageSize, "get")


proc extractText(documentId: DocumentId): Response =
  makeReq("/extracted_texts/" & $documentId, "post")

proc extractText*(document: Document): Response =
  extractText(document.id)

proc deleteExtractedText(documentId: DocumentId): Response =
  makeReq("/extracted_texts/" & $documentId, "delete")

proc deleteExtractedText*(document: Document): Response =
  deleteExtractedText(document.id)



export json, httpclient, strutils, math, sequtils
