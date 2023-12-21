/*
    normal comment
*//*
    also normal
*/
pragma solidity ^0.8.18;
/*
ga
*/
contract Foo {   
    /*//////// 
        BLAH
    /////*/
    uint256 s_1 = 0;
    /** ablah  */
    uint256 s_2 = 0;
    /*/// bbalh  "/*\/*\/** */     //" //sfsfs  

    uint256 s_3 = 0; // this is a side comment

    /*/// there is a space at the end of this line  //*/ 
    uint256 s_4 = 0; // scc-dblah

    // /*** ifjgs sfsf

    /**
     * 
     * 
     this is longer comment */ uint256 s_5 = 0;

    function foo(address bar) public pure {
        // blah ouhsff 
        // sdfssaf /*dsf

        require(bar != address(0));
    }

    function emoji() public pure {
        string memory a = /*sfgsgdf*/ unicode"Hello 😃";
    }

    function foo2(address bar) public pure {
        string memory simple = "/* this is not a \\\" simple comment */";
        string memory simple2 = "/* this is not \" /*a comment */";/* this is a comment */ uint256 m_6 = 0;
        uint256 m_47 = 0;
        uint256 m_217 = 0; /*//// sick! /////*/ uint256 m_917 = 0;
         /*//////// 
        OOBLAH
    /////*/
        string memory simple3 = '/* this is not a \\\' simple comment */'; // this semicolon no longer disappears !!
        // h haha sd
        string memory simple4 = '/* this is not \' /*a comment */';/* this is a comment */ uint256 m_7 = 0;
         /*//////// 
        ANOTHER ONE
    /////   */
        require(bar != address(0));

        string memory multiline = "sfgsfgqqsdf"
        "plkjksf"
        "nmbmbmbmnbsdfsfsfs";

        string memory smultiline = "sfgsfgqqsdf\
        plkjksf\
        nmbmbmbmnbsdfsfsfs";

        string memory smultiline2 = "sfgsfgqqsdf\
        \\/*notacomment*/plkjksf\
        nmbmbmbmnbsdfsfsfs";


    }
}/* 


// sdfsf"""""

*/