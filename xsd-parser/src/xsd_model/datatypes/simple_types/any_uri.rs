// xsd:anyURI
// The type xsd:anyURI represents a Uniform Resource Identifier (URI) reference. URIs are used to identify resources, and they may be absolute or relative. Absolute URIs provide the entire context for locating the resources, such as http://datypic.com/prod.html. Relative URIs are specified as the difference from a base URI, such as ../prod.html. It is also possible to specify a fragment identifier, using the # character, such as ../prod.html#shirt.
// The three previous examples happen to be HTTP URLs (Uniform Resource Locators), but URIs also encompass URLs of other schemes (e.g., FTP, gopher, telnet), as well as URNs (Uniform Resource Names). URIs are not required to be dereferencable; that is, it is not necessary for there to be a web page at http://datypic.com/prod.html in order for this to be a valid URI.
// URIs require that some characters be escaped with their hexadecimal Unicode code point preceded by the % character. This includes non-ASCII characters and some ASCII characters, namely control characters, spaces, and the following characters (unless they are used as deliimiters in the URI): <>#%{}|\^`. For example, ../édition.html must be represented instead as ../%C3%A9dition.html, with the é escaped as %C3%A9. However, the anyURI type will accept these characters either escaped or unescaped. With the exception of the characters % and #, it will assume that unescaped characters are intended to be escaped when used in an actual URI, although the schema processor will do nothing to alter them. It is valid for an anyURI value to contain a space, but this practice is strongly discouraged. Spaces should instead be escaped using %20.
// The schema processor is not required to parse the contents of an xsd:anyURI value to determine whether it is valid according to any particular URI scheme. Since the bare minimum rules for valid URI references are fairly generic, the schema processor will accept most character strings, including an empty value. The only values that are not accepted are ones that make inappropriate use of reserved characters, such as ones that contain multiple # characters or have % characters that are not followed by two hexadecimal digits.
// Note that when relative URI references such as "../prod" are used as values of xsd:anyURI, no attempt is made to determine or keep track of the base URI to which they may be applied. For more information on URIs, see RFC 2396, Uniform Resource Identifiers (URI): Generic Syntax.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:anySimpleType
// White Space: collapse
//
// Examples
// Valid values	                        Comment
// http://datypic.com	                    absolute URI (also a URL)
// mailto:info@datypic.com	                absolute URI
// ../%C3%A9dition.html	                    relative URI containing escaped non-ASCII character
// ../édition.html	                        relative URI containing unescaped non-ASCII character
// http://datypic.com/prod.html#shirt	    URI with fragment identifier
// ../prod.html#shirt	                    relative URI with fragment identifier
// urn:example:org	                        URN
//                                         an empty value is allowed
// Invalid values	                    Comment
// http://datypic.com#frag1#frag2	        too many # characters
// http://datypic.com#f% rag	            % character followed by something other than two hexadecimal digits
//
// Type Inheritance Chain
// xsd:anySimpleType
//  restricted by xsd:anyURI
pub struct AnyUri<'a>(&'a str);
