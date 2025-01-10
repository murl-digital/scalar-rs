== SCALAR EDITOR PROTOCOL ==

== DATA MODEL ==
A document can be in 2 possible states: Published, and Draft

A published document must be correct (deserializable and validated by the user defined schema), and have the published_at field set. A draft document has no correctness guaruntees


== PROTOCOL ==
POST <BASE_URL>/docs/<DOC::identifier>/drafts/<ID> -> creates a new draft with a given id
PATCH <BASE_URL>/docs/<DOC::identifier>/drafts/<ID> -> apply the JSON patch in the request body to the draft with a specified id

POST <BASE_URL>/docs/<DOC::identifier>/verify -> run a verification on the json document input
POST <BASE_URL>/docs/<DOC::identifier>/drafts/<ID>/publish -> convert a draft to a published document, it will be verified before being put in a published state
