fn main() {
    let input_stream: &str = 
    "qfmfhmhjmjggwbbvdvwvlvrrtsrsccwsslvlffjrrtprprjjvmmclmmghhddpvddclctcqtccgbgdbgdgsdgghqhtqtvvptvppwrwprpvrrrhpphththvhhrnnhnlnslnlhnhnhgnhnpnqqsmsgsllprlprrlzzqzffmzztctbtnbtthlttqvqcqmcmpcpbbczzbqbgghcghchhvwvllfrfnnbssfzsszpsplpglpprnpnfnbnhbnbtbzzvbvpbbhjjlzzbtbvbppczppbwppqwwnwlwccglgvgrgmmdwmwrmrppnfnhhhhqthqthqqrhrshhhqbhqqjgjvjllzvzbzhbbpttjsszvzqqtzzmbmddpldpdcdnccrmcrmmpwprplrrqssvddmpdmmwfwwlrljrrdsssmhsspnpffjggqllnzlnlhnnmddfrfpfbbvssjsrrznngcghgchcmhmrrrtzztjzzhchssslsmlmvvpwpqpjqjdddmsdmmtgtmgtglttfbbgrrcprrqffmmjnjttcmczzgbzggthhsttpggrmmgwwnpnqnqvnqvqppmlpmlpmpjpljljmmtpptfppfrppfdpfdppddmdttgzgzzdbzdzhzhnnsqssvmvbbpjjzwwvnwvvzmvzmzpmpttvrvccqddpgdppgmmthmthtggsfsbbvfbvfvhfvhvvpwpddqrqgqhghnnfmfbbwrbrgbgbvgbbdttffrddqbqpqzzmttlhhsqhsqhsqhhlplttpnpsstpthhpfhpfplprrgfrgffjppghgppghgdggjmmcgmccjvvsrvsrrwgrgmrrngrgttvbtbltthrthrttmfffjpfpssncnrngrrltrltlggjgcgllrzzllhwwjwrwgrgsrgrhrphrhhqwqsqmqlmqlmqmqrmrnnwnhwnhwhzwzjwwgbghhsjspszsznzfztfzfpzzlczztctsstctqtfqfqcfqqjrjccttmqqfpfdfnfwnffqbfbblpbpfpcfcwfccblbwwmqwqrrgprpccngnhghpppwmpplcppfrfjjgmmbzbhbcbzzgdgsdsvvqllzlppnfnlnlslsljlppcscqqfjjjwzwppfgfjjsvvsggjbjljpjpzjzrzjrzzfnfpnfpnfpnfnsnggmpggdllpmmrhhdqqppttgqqcsqsjsbjsjrsrqrbqqmbmcbmcbmbfmfvfqqdbdppmrprnrggmjjhnhbhdhbbfcbbcjjdhhwjwmjmssjswscswcwzccbgbqqmqgmmsdsjsbsdbsddvttjpppcqpcqcgqqslqsssczszrzvvrtvvjppswwhnnwlwtwhhwwzfwwpfpddlvvnvnnvlnntjtqjqjzzjttvvbqbhqbhbbbwnwhnwnppdbpdpvddrqdqjjlvlqqdfdhdjhdjdcjcrcjjggfmfvvfllvfvgglzllmhmzmdddfwddqjjqjfqfcfrrstrssptsstllrflfwwgswslwlbwbwjjvhvfvhfhffhsslwsllbnbblccbwwjqwqqdllrdrnrnffcbbqqpnqqdmdndtntvvrjvvsvmvgvnnmjlwgnjcwljgwnrwpqlztwrpmpgqtwlhrcwsrrhqhjhznrtpqfdnzbfqrzwslptdbdcnqvcllpjsfdvmzqwvzbpnmfcfcjnbmhtwhttjgtnczwctpdthhwmzvzrrgsnmbflgmszgsbvghbzgcmcmszgsbfmlmpbdspqlftmqrcjtmvgcrzznlfwjcbmddplsqrfflqnqfsldwhnncczdmfrrrsbjjqsdzrsgbdbwjbslfcqglsqfddhdsrcdrgqfqthgmfjvnfdfgdncfzpvqcpscnpmfgvqbfwszwzgmqvmcrdrwplfshdgqrchmccpqfznbmfvlhdpctlqgjslrwhjfjlmqfblgjrdlnzdtwlpwhnrhrcrpfwqpmjlgrdbgpbljntmbqlblqqqpgrnjtmjqvjpzvsqdpgtchmmwbhtmgcjqdplrtptqcvdjjpqdzsrcjhcwvdcghlwrdhtdfctmqfcjcqhcvvbzgsvlggcrdgqbtznwwmnbgsfrjprqgcmlswftlwpqqqvshdprldrsghmhrqvmqmvglbvzpvtrjbhcvhqmvdtcvsllznqzjmhpnlbhmlzthbwwhhvdtcdfdcdzhnbsrnqqjvzzsvfjhbsdlsbdlqjnlpnhfcjtdppzmphghltztzcdvzwbftbvwhvgmrllqfzrpbltptdtjjqtfwjfmczzgdvclqbsbftgtlhnhrrvbpvdltstdnhqvpvtjhmghptvsfnlspslmfsftzdrwljrgblgmcbmlszmhnlfdtmsrnjqwrfmsnfgpcqgzmlwppffrmbvhnlstfpgzwwmwffrqpdfvrspbczbrclwljgzfhpsrwwpdndfgjwbjtftnjrqvmtmzvjmtlmjhhptmgjvfrlzncmhnmpfcwpjbcpftqfzvmtldqhjpwvzrdnvnwnscgzslvfgjjpcvjshctmmpjbgdwtdjtlmztsbmwrjtmltnlsmwmjnpcgpprnfwcqdldbbqbfmdnvprzqwvntgzdbrsgdpgdjbcblmqpdphmwgvbgwlpblflphvjgjsjfshbjdftcqmsdnrzbgngcvddddjvrndhdcscqqswrnvslfrlvvncqjhzlbhdqhtrlvdsvjsbglhfzfphmzfmzqdvjqdwhjgfdwmzsdmbjzstjddfmfqjhmbdgdbvvhbqgstrzpvhpthhbwljczzrmvgsmbqvzdrmhvvjlmphzjfbmfqvwhtnrlfnfmqnnjvnwjswzshwgljmfjhrwbwgtpdqnqgqdzbssbjfbsgwmfzpfjdrtrnmsdffhnbgnrdlbjzfjrvtjgjgcvvzgllljrcrshczvpfqgnwnjjnhbwgvzwrptrgrdgtczjfzzndsqhqpmtqsvmcncfszsjllzzsjjmwgplpjwlhnhgbhctrttgzqbbcflzqvqgmhgdtlvfpbtncbwsjgnzpmbspcqzzwfplfprqlnbctwwrzpjtpfrmnpvnjrjppqrzjrcmggfmhrstzhmsjllcgjhwrbhcrvdvgmvjqqgmczlmhstmthzphlvrrvqmhjzzfzbhphstflhfjdlwqvzlsszctrdchwjssdfjjfzszlqdtwwthfjdqprpfftgdrpdhhcsdcpjbhdrgzwbgjspmffcmgcjnpmwsqwsvpfwzddlcpvlgpvctrssghndhvdmmmgndcjvhdjwttqphsjpgfbsdczmplfpwpzzjlbhrjptmsshfttnmhzdzmjctbltqjmfnpndqgwjzwdwrgdjdmcbtvjqwjngrtbfrwcttpdvcqtwqndznbchjqcqttrhjpjgwdbwzvwgmdsdfmpdwctvntvnsdmfnznfrsdcllpgpnstrrfrwrfrwnhbclnqhltrcdwqwzzldgbbtzmcvnbzmwcmntqpbscqrpzcjnbgbrzpcrcmdmdfsfgdpmgvwccqjrltrgfvjdgbhjndnmtnjjhzvghscdhnhflwplrqdzrnlnsvrtrdnphgqwjwqcjvtfdfshqdwbsvgrqbdlncjmhdmrlsvdnrhztznczzllsvpqlvwgqjvgvvwgrjcvtjvhrsgbdgvlmmtjbwrnftzphnqslcpggztgsdbjsbdtzwprsbcljpbwjhcrffnvtplcdlgmbtcgbllbdmwhwcllbqstnqqvdbcjrglwbmcfqvlvtpqncbspbphflvvrrsprlhqspfmqrsdtdlftsfzrqwdfffbhccvpfdtlptqzllfsbbrfnhjgwhlfcwmmjgjndcwfhdzvvvrzmwllthwsdmbbsrfrzmqnlnqnjnfpgfvrhsbzhjftmvzrzpqpmlcbnwmbssmvssmmqpvwnsjppdhmnhpntlvqmjnbmtvjnmtbpbzrcfhjfhvztnwrmthbswwthjddjmsdnjmzhhpjdllgscdrgmhfpljfzsmszqsqqgrznddhfmstzdcqpgztgwwqpvrghtmqlgdddlqqwwwtnpldbqtf";
    

    // iterate all characters, keeping a tally of which character is currently being interrogated.
    fn shift_char_to_buffer(buffer: &[char;14], character: char) -> [char;14]{
        //[buffer[1],buffer[2],buffer[3],character]
        let mut outputs: [char;14] = [' ';14];
        let mut i: usize = 0;
        while i < buffer.len() - 1{
            outputs[i] = buffer[i + 1];
            i = i + 1;
        }
        outputs[buffer.len() - 1] = character;
        outputs
    }

    // returns true when buffer does not have a space in the first position.
    fn check_if_buffer_is_full(buffer: &[char;14]) -> bool{
        match buffer[0]{
            ' ' => false,
            _ => true
        }
    }

    // returns true unless there is a duplicate
    fn check_buffer_has_no_duplicates(buffer: &[char;14]) -> bool{
        // buffer[0] != buffer[1] &&
        // buffer[0] != buffer[2] &&
        // buffer[0] != buffer[3] &&
        // buffer[1] != buffer[2] &&
        // buffer[1] != buffer[3] &&
        // buffer[2] != buffer[3]
        let local_buffer = &buffer.clone();
        for character in local_buffer{
            let loop_buffer = local_buffer.to_vec();
            let count = loop_buffer.iter().filter(|current| **current == *character).count();
            //println!("DEBUG count: {}", count);
            if count > 1{
                return false;
            }
        }
        true
    }

    // Keep a buffer of the last 4 characters being interrogated. If they're all unique, report the tally value

    fn find_end_of_first_packet_signal(signal: &str) -> Option<usize> {
        let mut buffer: [char;14] = [' ';14];
        let chars = signal.chars().collect::<Vec<char>>();
        let mut index: usize = 0;
        println!("Finding end in list of {} characters...", signal.len());
        while index < signal.len() {
            let character_from_index = chars[index];
            buffer = shift_char_to_buffer(&buffer, character_from_index);
            if ! check_if_buffer_is_full(&buffer){
                index = index + 1;
                continue;
            }
            if check_buffer_has_no_duplicates(&buffer){
                index = index + 1;
                break;
            }
            index = index + 1;
        }

        if index == signal.len(){
            return None;
        }

        Some(index)
    }

    fn safely_report_end_location(location: Option<usize>){
        match location{
            Some(value) => {
                println!("First packet found after character number: {}.", value);
            },
            None => {
                println!("No first packet found.");
            }
        }
    }


    // TODO: Call functions to get result

    let end = find_end_of_first_packet_signal(input_stream);
    safely_report_end_location(end);


}
