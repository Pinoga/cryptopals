static BASE_64_CHARACTERS: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn base_64_encode_bytes(input: &[u8]) -> Vec<u8> {
    let size: usize = (input.len().div_ceil(3) * 4) as usize;

    let mut output = Vec::with_capacity(size);

    let mut index: usize = 0;

    // We have at least 3 bytes to read
    while index + 2 < input.len() {
        // load 3 bytes into chunk
        let chunk = ((input[index] as u32) << 16)
            | ((input[index + 1] as u32) << 8)
            | (input[index + 2] as u32);

        // read 6 bits
        output.push(BASE_64_CHARACTERS[((chunk >> 18) & 0x3F) as usize]);
        // read 6 bits
        output.push(BASE_64_CHARACTERS[((chunk >> 12) & 0x3F) as usize]);
        // read 6 bits
        output.push(BASE_64_CHARACTERS[((chunk >> 6) & 0x3F) as usize]);
        // read 6 bits
        output.push(BASE_64_CHARACTERS[(chunk & 0x3F) as usize]);

        index += 3;
    }

    let remaining_bytes = input.len() - index;

    if remaining_bytes == 1 {
        let chunk = (input[index] as u32) << 16;

        // read 6 bits
        output.push(BASE_64_CHARACTERS[((chunk >> 18) & 0x3F) as usize]);

        // read 6 bits (with 4-bit padding)
        output.push(BASE_64_CHARACTERS[((chunk >> 12) & 0x3F) as usize]);

        // since we've formed only two letters, pad the output with two ='s
        output.push(b'=');
        output.push(b'=');
    } else if remaining_bytes == 2 {
        let chunk = ((input[index] as u32) << 16) | ((input[index + 1] as u32) << 8);

        // read 6 bits
        output.push(BASE_64_CHARACTERS[((chunk >> 18) & 0x3F) as usize]);

        // read 6 bits
        output.push(BASE_64_CHARACTERS[((chunk >> 12) & 0x3F) as usize]);

        // read 6 bits (with 2-bit padding)
        output.push(BASE_64_CHARACTERS[((chunk >> 6) & 0x3F) as usize]);

        // since we've formed only three letters, pad the output with a =
        output.push(b'=');
    }

    return output;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_input() {
        let input: [u8; 0] = [];
        let result = base_64_encode_bytes(&input);
        assert_eq!(result, []);
    }

    #[test]
    fn one_character() {
        let input = [b'0'];
        let result = base_64_encode_bytes(&input);
        assert_eq!(result, b"MA==");
    }

    #[test]
    fn two_characters() {
        let input = [b'0', b'1'];
        let result = base_64_encode_bytes(&input);
        assert_eq!(result, b"MDE=");
    }

    #[test]
    fn three_characters() {
        let input = [b'0', b'1', b'2'];
        let result = base_64_encode_bytes(&input);
        assert_eq!(result, b"MDEy");
    }

    #[test]
    fn large_input() {
        let input = b"New had happen unable uneasy. Drawings can followed improved out sociable not. Earnestly so do instantly pretended. See general few civilly amiable pleased account carried. Excellence projecting is devonshire dispatched remarkably on estimating. Side in so life past. Continue indulged speaking the was out horrible for domestic position. Seeing rather her you not esteem men settle genius excuse. Deal say over you age from. Comparison new ham melancholy son themselves.Oh he decisively impression attachment friendship so if everything. Whose her enjoy chief new young. Felicity if ye required likewise so doubtful. On so attention necessary at by provision otherwise existence direction. Unpleasing up announcing unpleasant themselves oh do on. Way advantage age led listening belonging supposing.Those an equal point no years do. Depend warmth fat but her but played. Shy and subjects wondered trifling pleasant. Prudent cordial comfort do no on colonel as assured chicken. Smart mrs day which begin. Snug do sold mr it if such. Terminated uncommonly at at estimating. Man behaviour met moonlight extremity acuteness direction.Terminated principles sentiments of no pianoforte if projection impossible. Horses pulled nature favour number yet highly his has old. Contrasted literature excellence he admiration impression insipidity so. Scale ought who terms after own quick since. Servants margaret husbands to screened in throwing. Imprudence oh an collecting partiality. Admiration gay difficulty unaffected how.Ought these are balls place mrs their times add she. Taken no great widow spoke of it small. Genius use except son esteem merely her limits. Sons park by do make on. It do oh cottage offered cottage in written. Especially of dissimilar up attachment themselves by interested boisterous. Linen mrs seems men table. Jennings dashwood to quitting marriage bachelor in. On as conviction in of appearance apartments boisterous.However venture pursuit he am mr cordial. Forming musical am hearing studied be luckily. Ourselves for determine attending how led gentleman sincerity. Valley afford uneasy joy she thrown though bed set. In me forming general prudent on country carried. Behaved an or suppose justice. Seemed whence how son rather easily and change missed. Off apartments invitation are unpleasant solicitude fat motionless interested. Hardly suffer wisdom wishes valley as an. As friendship advantages resolution it alteration stimulated he or increasing.Much evil soon high in hope do view. Out may few northward believing attempted. Yet timed being songs marry one defer men our. Although finished blessing do of. Consider speaking me prospect whatever if. Ten nearer rather hunted six parish indeed number. Allowance repulsive sex may contained can set suspected abilities cordially. Do part am he high rest that. So fruit to ready it being views match.On am we offices expense thought. Its hence ten smile age means. Seven chief sight far point any. Of so high into easy. Dashwoods eagerness oh extensive as discourse sportsman frankness. Husbands see disposed surprise likewise humoured yet pleasure. Fifteen no inquiry cordial so resolve garrets as. Impression was estimating surrounded solicitude indulgence son shy.Admiration stimulated cultivated reasonable be projection possession of. Real no near room ye bred sake if some. Is arranging furnished knowledge agreeable so. Fanny as smile up small. It vulgar chatty simple months turned oh at change of. Astonished set expression solicitude way admiration.Is branched in my up strictly remember. Songs but chief has ham widow downs. Genius or so up vanity cannot. Large do tried going about water defer by. Silent son man she wished mother. Distrusts allowance do knowledge eagerness assurance additions to.";
        let result = base_64_encode_bytes(input);
        assert_eq!(result, b"TmV3IGhhZCBoYXBwZW4gdW5hYmxlIHVuZWFzeS4gRHJhd2luZ3MgY2FuIGZvbGxvd2VkIGltcHJvdmVkIG91dCBzb2NpYWJsZSBub3QuIEVhcm5lc3RseSBzbyBkbyBpbnN0YW50bHkgcHJldGVuZGVkLiBTZWUgZ2VuZXJhbCBmZXcgY2l2aWxseSBhbWlhYmxlIHBsZWFzZWQgYWNjb3VudCBjYXJyaWVkLiBFeGNlbGxlbmNlIHByb2plY3RpbmcgaXMgZGV2b25zaGlyZSBkaXNwYXRjaGVkIHJlbWFya2FibHkgb24gZXN0aW1hdGluZy4gU2lkZSBpbiBzbyBsaWZlIHBhc3QuIENvbnRpbnVlIGluZHVsZ2VkIHNwZWFraW5nIHRoZSB3YXMgb3V0IGhvcnJpYmxlIGZvciBkb21lc3RpYyBwb3NpdGlvbi4gU2VlaW5nIHJhdGhlciBoZXIgeW91IG5vdCBlc3RlZW0gbWVuIHNldHRsZSBnZW5pdXMgZXhjdXNlLiBEZWFsIHNheSBvdmVyIHlvdSBhZ2UgZnJvbS4gQ29tcGFyaXNvbiBuZXcgaGFtIG1lbGFuY2hvbHkgc29uIHRoZW1zZWx2ZXMuT2ggaGUgZGVjaXNpdmVseSBpbXByZXNzaW9uIGF0dGFjaG1lbnQgZnJpZW5kc2hpcCBzbyBpZiBldmVyeXRoaW5nLiBXaG9zZSBoZXIgZW5qb3kgY2hpZWYgbmV3IHlvdW5nLiBGZWxpY2l0eSBpZiB5ZSByZXF1aXJlZCBsaWtld2lzZSBzbyBkb3VidGZ1bC4gT24gc28gYXR0ZW50aW9uIG5lY2Vzc2FyeSBhdCBieSBwcm92aXNpb24gb3RoZXJ3aXNlIGV4aXN0ZW5jZSBkaXJlY3Rpb24uIFVucGxlYXNpbmcgdXAgYW5ub3VuY2luZyB1bnBsZWFzYW50IHRoZW1zZWx2ZXMgb2ggZG8gb24uIFdheSBhZHZhbnRhZ2UgYWdlIGxlZCBsaXN0ZW5pbmcgYmVsb25naW5nIHN1cHBvc2luZy5UaG9zZSBhbiBlcXVhbCBwb2ludCBubyB5ZWFycyBkby4gRGVwZW5kIHdhcm10aCBmYXQgYnV0IGhlciBidXQgcGxheWVkLiBTaHkgYW5kIHN1YmplY3RzIHdvbmRlcmVkIHRyaWZsaW5nIHBsZWFzYW50LiBQcnVkZW50IGNvcmRpYWwgY29tZm9ydCBkbyBubyBvbiBjb2xvbmVsIGFzIGFzc3VyZWQgY2hpY2tlbi4gU21hcnQgbXJzIGRheSB3aGljaCBiZWdpbi4gU251ZyBkbyBzb2xkIG1yIGl0IGlmIHN1Y2guIFRlcm1pbmF0ZWQgdW5jb21tb25seSBhdCBhdCBlc3RpbWF0aW5nLiBNYW4gYmVoYXZpb3VyIG1ldCBtb29ubGlnaHQgZXh0cmVtaXR5IGFjdXRlbmVzcyBkaXJlY3Rpb24uVGVybWluYXRlZCBwcmluY2lwbGVzIHNlbnRpbWVudHMgb2Ygbm8gcGlhbm9mb3J0ZSBpZiBwcm9qZWN0aW9uIGltcG9zc2libGUuIEhvcnNlcyBwdWxsZWQgbmF0dXJlIGZhdm91ciBudW1iZXIgeWV0IGhpZ2hseSBoaXMgaGFzIG9sZC4gQ29udHJhc3RlZCBsaXRlcmF0dXJlIGV4Y2VsbGVuY2UgaGUgYWRtaXJhdGlvbiBpbXByZXNzaW9uIGluc2lwaWRpdHkgc28uIFNjYWxlIG91Z2h0IHdobyB0ZXJtcyBhZnRlciBvd24gcXVpY2sgc2luY2UuIFNlcnZhbnRzIG1hcmdhcmV0IGh1c2JhbmRzIHRvIHNjcmVlbmVkIGluIHRocm93aW5nLiBJbXBydWRlbmNlIG9oIGFuIGNvbGxlY3RpbmcgcGFydGlhbGl0eS4gQWRtaXJhdGlvbiBnYXkgZGlmZmljdWx0eSB1bmFmZmVjdGVkIGhvdy5PdWdodCB0aGVzZSBhcmUgYmFsbHMgcGxhY2UgbXJzIHRoZWlyIHRpbWVzIGFkZCBzaGUuIFRha2VuIG5vIGdyZWF0IHdpZG93IHNwb2tlIG9mIGl0IHNtYWxsLiBHZW5pdXMgdXNlIGV4Y2VwdCBzb24gZXN0ZWVtIG1lcmVseSBoZXIgbGltaXRzLiBTb25zIHBhcmsgYnkgZG8gbWFrZSBvbi4gSXQgZG8gb2ggY290dGFnZSBvZmZlcmVkIGNvdHRhZ2UgaW4gd3JpdHRlbi4gRXNwZWNpYWxseSBvZiBkaXNzaW1pbGFyIHVwIGF0dGFjaG1lbnQgdGhlbXNlbHZlcyBieSBpbnRlcmVzdGVkIGJvaXN0ZXJvdXMuIExpbmVuIG1ycyBzZWVtcyBtZW4gdGFibGUuIEplbm5pbmdzIGRhc2h3b29kIHRvIHF1aXR0aW5nIG1hcnJpYWdlIGJhY2hlbG9yIGluLiBPbiBhcyBjb252aWN0aW9uIGluIG9mIGFwcGVhcmFuY2UgYXBhcnRtZW50cyBib2lzdGVyb3VzLkhvd2V2ZXIgdmVudHVyZSBwdXJzdWl0IGhlIGFtIG1yIGNvcmRpYWwuIEZvcm1pbmcgbXVzaWNhbCBhbSBoZWFyaW5nIHN0dWRpZWQgYmUgbHVja2lseS4gT3Vyc2VsdmVzIGZvciBkZXRlcm1pbmUgYXR0ZW5kaW5nIGhvdyBsZWQgZ2VudGxlbWFuIHNpbmNlcml0eS4gVmFsbGV5IGFmZm9yZCB1bmVhc3kgam95IHNoZSB0aHJvd24gdGhvdWdoIGJlZCBzZXQuIEluIG1lIGZvcm1pbmcgZ2VuZXJhbCBwcnVkZW50IG9uIGNvdW50cnkgY2FycmllZC4gQmVoYXZlZCBhbiBvciBzdXBwb3NlIGp1c3RpY2UuIFNlZW1lZCB3aGVuY2UgaG93IHNvbiByYXRoZXIgZWFzaWx5IGFuZCBjaGFuZ2UgbWlzc2VkLiBPZmYgYXBhcnRtZW50cyBpbnZpdGF0aW9uIGFyZSB1bnBsZWFzYW50IHNvbGljaXR1ZGUgZmF0IG1vdGlvbmxlc3MgaW50ZXJlc3RlZC4gSGFyZGx5IHN1ZmZlciB3aXNkb20gd2lzaGVzIHZhbGxleSBhcyBhbi4gQXMgZnJpZW5kc2hpcCBhZHZhbnRhZ2VzIHJlc29sdXRpb24gaXQgYWx0ZXJhdGlvbiBzdGltdWxhdGVkIGhlIG9yIGluY3JlYXNpbmcuTXVjaCBldmlsIHNvb24gaGlnaCBpbiBob3BlIGRvIHZpZXcuIE91dCBtYXkgZmV3IG5vcnRod2FyZCBiZWxpZXZpbmcgYXR0ZW1wdGVkLiBZZXQgdGltZWQgYmVpbmcgc29uZ3MgbWFycnkgb25lIGRlZmVyIG1lbiBvdXIuIEFsdGhvdWdoIGZpbmlzaGVkIGJsZXNzaW5nIGRvIG9mLiBDb25zaWRlciBzcGVha2luZyBtZSBwcm9zcGVjdCB3aGF0ZXZlciBpZi4gVGVuIG5lYXJlciByYXRoZXIgaHVudGVkIHNpeCBwYXJpc2ggaW5kZWVkIG51bWJlci4gQWxsb3dhbmNlIHJlcHVsc2l2ZSBzZXggbWF5IGNvbnRhaW5lZCBjYW4gc2V0IHN1c3BlY3RlZCBhYmlsaXRpZXMgY29yZGlhbGx5LiBEbyBwYXJ0IGFtIGhlIGhpZ2ggcmVzdCB0aGF0LiBTbyBmcnVpdCB0byByZWFkeSBpdCBiZWluZyB2aWV3cyBtYXRjaC5PbiBhbSB3ZSBvZmZpY2VzIGV4cGVuc2UgdGhvdWdodC4gSXRzIGhlbmNlIHRlbiBzbWlsZSBhZ2UgbWVhbnMuIFNldmVuIGNoaWVmIHNpZ2h0IGZhciBwb2ludCBhbnkuIE9mIHNvIGhpZ2ggaW50byBlYXN5LiBEYXNod29vZHMgZWFnZXJuZXNzIG9oIGV4dGVuc2l2ZSBhcyBkaXNjb3Vyc2Ugc3BvcnRzbWFuIGZyYW5rbmVzcy4gSHVzYmFuZHMgc2VlIGRpc3Bvc2VkIHN1cnByaXNlIGxpa2V3aXNlIGh1bW91cmVkIHlldCBwbGVhc3VyZS4gRmlmdGVlbiBubyBpbnF1aXJ5IGNvcmRpYWwgc28gcmVzb2x2ZSBnYXJyZXRzIGFzLiBJbXByZXNzaW9uIHdhcyBlc3RpbWF0aW5nIHN1cnJvdW5kZWQgc29saWNpdHVkZSBpbmR1bGdlbmNlIHNvbiBzaHkuQWRtaXJhdGlvbiBzdGltdWxhdGVkIGN1bHRpdmF0ZWQgcmVhc29uYWJsZSBiZSBwcm9qZWN0aW9uIHBvc3Nlc3Npb24gb2YuIFJlYWwgbm8gbmVhciByb29tIHllIGJyZWQgc2FrZSBpZiBzb21lLiBJcyBhcnJhbmdpbmcgZnVybmlzaGVkIGtub3dsZWRnZSBhZ3JlZWFibGUgc28uIEZhbm55IGFzIHNtaWxlIHVwIHNtYWxsLiBJdCB2dWxnYXIgY2hhdHR5IHNpbXBsZSBtb250aHMgdHVybmVkIG9oIGF0IGNoYW5nZSBvZi4gQXN0b25pc2hlZCBzZXQgZXhwcmVzc2lvbiBzb2xpY2l0dWRlIHdheSBhZG1pcmF0aW9uLklzIGJyYW5jaGVkIGluIG15IHVwIHN0cmljdGx5IHJlbWVtYmVyLiBTb25ncyBidXQgY2hpZWYgaGFzIGhhbSB3aWRvdyBkb3ducy4gR2VuaXVzIG9yIHNvIHVwIHZhbml0eSBjYW5ub3QuIExhcmdlIGRvIHRyaWVkIGdvaW5nIGFib3V0IHdhdGVyIGRlZmVyIGJ5LiBTaWxlbnQgc29uIG1hbiBzaGUgd2lzaGVkIG1vdGhlci4gRGlzdHJ1c3RzIGFsbG93YW5jZSBkbyBrbm93bGVkZ2UgZWFnZXJuZXNzIGFzc3VyYW5jZSBhZGRpdGlvbnMgdG8u");
    }
}
