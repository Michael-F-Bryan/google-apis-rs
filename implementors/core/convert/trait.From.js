(function() {var implementors = {};
implementors['rustc_serialize'] = ["<a class='stability Stable' title='Stable'></a>impl <a class='trait' href='http://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='enum' href='rustc_serialize/json/enum.ParserError.html' title='rustc_serialize::json::ParserError'>ParserError</a>&gt; for <a class='enum' href='rustc_serialize/json/enum.DecoderError.html' title='rustc_serialize::json::DecoderError'>DecoderError</a>","<a class='stability Stable' title='Stable'></a>impl <a class='trait' href='http://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='struct' href='http://doc.rust-lang.org/nightly/std/io/error/struct.Error.html' title='std::io::error::Error'>Error</a>&gt; for <a class='enum' href='rustc_serialize/json/enum.ParserError.html' title='rustc_serialize::json::ParserError'>ParserError</a>","<a class='stability Stable' title='Stable'></a>impl <a class='trait' href='http://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='struct' href='http://doc.rust-lang.org/nightly/core/fmt/struct.Error.html' title='core::fmt::Error'>Error</a>&gt; for <a class='enum' href='rustc_serialize/json/enum.EncoderError.html' title='rustc_serialize::json::EncoderError'>EncoderError</a>",];implementors['num'] = ["<a class='stability Stable' title='Stable'></a>impl <a class='trait' href='http://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='struct' href='http://doc.rust-lang.org/nightly/core/num/struct.ParseIntError.html' title='core::num::ParseIntError'>ParseIntError</a>&gt; for <a class='enum' href='num/bigint/enum.ParseBigIntError.html' title='num::bigint::ParseBigIntError'>ParseBigIntError</a>",];implementors['serde'] = ["<a class='stability Stable' title='Stable'></a>impl&lt;'a&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a href='http://doc.rust-lang.org/nightly/std/primitive.slice.html'>&amp;'a [</a><a href='http://doc.rust-lang.org/nightly/std/primitive.u8.html'>u8</a><a href='http://doc.rust-lang.org/nightly/std/primitive.slice.html'>]</a>&gt; for <a class='struct' href='serde/bytes/struct.Bytes.html' title='serde::bytes::Bytes'>Bytes</a>&lt;'a&gt;","<a class='stability Stable' title='Stable'></a>impl&lt;'a&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;&amp;'a <a class='struct' href='http://doc.rust-lang.org/nightly/collections/vec/struct.Vec.html' title='collections::vec::Vec'>Vec</a>&lt;<a href='http://doc.rust-lang.org/nightly/std/primitive.u8.html'>u8</a>&gt;&gt; for <a class='struct' href='serde/bytes/struct.Bytes.html' title='serde::bytes::Bytes'>Bytes</a>&lt;'a&gt;","<a class='stability Stable' title='Stable'></a>impl <a class='trait' href='http://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='struct' href='http://doc.rust-lang.org/nightly/collections/vec/struct.Vec.html' title='collections::vec::Vec'>Vec</a>&lt;<a href='http://doc.rust-lang.org/nightly/std/primitive.u8.html'>u8</a>&gt;&gt; for <a class='struct' href='serde/bytes/struct.ByteBuf.html' title='serde::bytes::ByteBuf'>ByteBuf</a>","<a class='stability Stable' title='Stable'></a>impl <a class='trait' href='http://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='struct' href='http://doc.rust-lang.org/nightly/std/io/error/struct.Error.html' title='std::io::error::Error'>Error</a>&gt; for <a class='enum' href='serde/json/error/enum.Error.html' title='serde::json::error::Error'>Error</a>","<a class='stability Stable' title='Stable'></a>impl <a class='trait' href='http://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='enum' href='serde/de/value/enum.Error.html' title='serde::de::value::Error'>Error</a>&gt; for <a class='enum' href='serde/json/error/enum.Error.html' title='serde::json::error::Error'>Error</a>",];implementors['hyper'] = ["<a class='stability Stable' title='Stable'></a>impl&lt;'a, R: <a class='trait' href='http://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a>&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;&amp;'a mut R&gt; for <a class='enum' href='hyper/client/enum.Body.html' title='hyper::client::Body'>Body</a>&lt;'a&gt;","<a class='stability Stable' title='Stable'></a>impl <a class='trait' href='http://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='struct' href='http://doc.rust-lang.org/nightly/std/io/error/struct.Error.html' title='std::io::error::Error'>IoError</a>&gt; for <a class='enum' href='hyper/error/enum.Error.html' title='hyper::error::Error'>Error</a>","<a class='stability Stable' title='Stable'></a>impl <a class='trait' href='http://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='enum' href='url/parser/enum.ParseError.html' title='url::parser::ParseError'>ParseError</a>&gt; for <a class='enum' href='hyper/error/enum.Error.html' title='hyper::error::Error'>Error</a>","<a class='stability Stable' title='Stable'></a>impl <a class='trait' href='http://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='enum' href='openssl/ssl/error/enum.SslError.html' title='openssl::ssl::error::SslError'>SslError</a>&gt; for <a class='enum' href='hyper/error/enum.Error.html' title='hyper::error::Error'>Error</a>","<a class='stability Stable' title='Stable'></a>impl <a class='trait' href='http://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='enum' href='httparse/enum.Error.html' title='httparse::Error'>Error</a>&gt; for <a class='enum' href='hyper/error/enum.Error.html' title='hyper::error::Error'>Error</a>","<a class='stability Stable' title='Stable'></a>impl&lt;T: <a class='trait' href='hyper/net/trait.NetworkStream.html' title='hyper::net::NetworkStream'>NetworkStream</a> + <a class='trait' href='http://doc.rust-lang.org/nightly/core/marker/trait.Send.html' title='core::marker::Send'>Send</a>&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;T&gt; for <a class='struct' href='http://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html' title='alloc::boxed::Box'>Box</a>&lt;<a class='trait' href='hyper/net/trait.NetworkStream.html' title='hyper::net::NetworkStream'>NetworkStream</a> + <a class='trait' href='http://doc.rust-lang.org/nightly/core/marker/trait.Send.html' title='core::marker::Send'>Send</a>&gt;",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
