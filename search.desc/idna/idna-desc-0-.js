searchState.loadedDescShard("idna", 0, "This Rust crate implements IDNA per the WHATWG URL Standard…\nDeprecated configuration API.\nType indicating that there were errors during UTS #46 …\nDeprecated. Use the crate-top-level functions or <code>Uts46</code>.\nWhether to enforce STD3 rules for hyphen placement.\nThe domain to ASCII algorithm; version returning <code>String</code> …\nThe domain to ASCII algorithm; version returning a <code>Cow</code>.\nThe domain to ASCII algorithm, with the <code>beStrict</code> flag set.\nThe domain to Unicode algorithm; version returning <code>String</code> …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nPunycode (RFC 3492) implementation.\nUTS 46 ToASCII\nUTS 46 ToASCII\nUTS 46 ToUnicode\nUTS 46 ToUnicode\nWhether to enable (deprecated) transitional processing.\nObsolete method retained to ease migration. The argument …\nWhether to enforce STD3 or WHATWG URL Standard ASCII deny …\nThis module provides the lower-level API for UTS 46.\nWhether the <em>VerifyDNSLength</em> operation should be performed …\nConvert Punycode to Unicode.\nConvert Punycode to an Unicode <code>String</code>.\nConvert Unicode to Punycode.\nConvert an Unicode <code>str</code> to Punycode.\n<em>CheckHyphens=false</em>: Do not place positional restrictions …\nThe ASCII deny list to be applied.\n<em>CheckHyphens=true</em>: Prohibit hyphens in the first, third, …\nProhibit hyphens in the first and last position in the …\nThe UTS 46 <em>VerifyDNSLength</em> flag.\nNo ASCII deny list. This corresponds to …\nPolicy for customizing behavior in case of an error.\nReturn as early as possible without producing output in …\nThe <em>CheckHyphens</em> mode.\n<em>VerifyDNSLength=false</em>. (Possibly relevant for allowing …\nIn case of error, mark errors with the REPLACEMENT …\nThere were no errors. The caller must consider the input …\nThe failure outcome of <code>Uts46::process</code>\nThe success outcome of <code>Uts46::process</code>\nThe STD3 deny list. This corresponds to …\nThe sink emitted <code>core::fmt::Error</code>. The partial output …\nForbidden domain code point from the WHATWG URL Standard.\nAn implementation of UTS #46.\nThere was a validity error according to the chosen options.\n<em>VerifyDNSLength=true</em>. (The trailing root label dot is not …\n<em>VerifyDNSLength=true</em> with the exception that the trailing …\nThere were no errors. The caller must consider what was …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConstructor using data compiled into the binary.\nComputes (preferably at compile time) an ASCII deny list.\nThe lower-level function that <code>Uts46::to_ascii</code>, …\nPerforms the ToASCII operation from UTS #46 with the …\nPerforms the ToUnicode operation from UTS #46 according to …\nPerforms the ToUnicode operation from UTS #46 according to …\nPerforms the <em>VerifyDNSLength</em> check on the output of the …")