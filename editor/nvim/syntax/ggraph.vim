if exists("b:current_syntax")
  finish
endif

" Comments
syntax region GgraphComment start="%" end="%"

" String literals
syntax region GgraphString start=+"+ end=+"+ keepend contains=GgraphEscape

" Number literals 0xff, 0123, 0.23 or .23
syntax match GgraphNumber /\v(0x[0-9a-fA-F]+|0\d+|\d+(\.\d+)?)/

" Inner Functions @function_name
syntax match GgraphFunction /@\w\+/

" Top level Elements #define, #root, #grid
syntax match GgraphRoot /#root/
syntax match GgraphDefine /#define/
syntax match GgraphGrid /#grid/
syntax match GgraphEnd /#end/





" Set highlights
highlight default link GgraphComment Comment
highlight default link GgraphString String
highlight default link GgraphNumber Number
highlight default link GgraphFunction Function
highlight default link GgraphElement Keyword
highlight default link GgraphRoot Keyword
highlight default link GgraphDefine Keyword
highlight default link GgraphGrid Keyword
highlight default link GgraphEnd Keyword
