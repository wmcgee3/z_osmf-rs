#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let datasets_client = _setup::get_zosmf().await?.datasets();

    let data = r#"
/******************************************************/
/* THIS PARMLIB MEMBER CONTAINS CONFIGURATION FOR SMF */
/******************************************************/
 ACTIVE                       /*ACTIVE SMF RECORDING*/              00010000
 DSNAME(SYS1.&SMFDSN1,SYS1.&SMFDSN2,    /*SMF ON 3390        */     00020000
  SYS1.&SMFDSN3)                    /*FT: SYSAQ3, TS: SYSAQ4 */     00030000
 NOPROMPT                     /*PROMPT THE OPERATOR FOR OPTIONS*/   00040000
 REC(PERM)                    /*TYPE 17 PERM RECORDS ONLY*/         00050000
 MAXDORM(3000)               /* WRITE AN IDLE BUFFER AFTER 30 MIN*/ 00060000
 MEMLIMIT(256M)              /* 256M FOR 64 BIT APPS             */ 00061005
 STATUS(003000)              /* WRITE SMF STATS AFTER HALF HOUR*/   00070000
 JWT(0700)                   /* INVOKE EXIT IEFUTL AFTER  7HR 00M*/ 00080002
 SID(&SYSNAME),              /* SYSTEM ID FOR 3084 - SINGLE IMAGE*/ 00090000
 LISTDSN                     /* LIST DATA SET STATUS AT IPL*/       00100000
 INTVAL(30)                  /* INTVAL OPTION SP430 */              00110000
 SYNCVAL(00)                 /* SYNCVAL OPTION SP430  */            00120000
 SYS(NOTYPE(19,40,92),                                              00130001
  EXITS(IEFU83,IEFU84,IEFACTRT,IEFUJV,IEFUJI,                       00140000
       IEFUSI,IEFUTL,IEFU29),INTERVAL(010000),DETAIL)               00150000
                                                                    00160000
 /* WRITE ALL RECORDS AS THE SYSTEM DEFAULT, TAKE ALL KNOWN         00170000
    EXITS, NOTE: JES EXITS CONTROLED BY JES , THERE IS NO           00180000
    DEFAULT INTERVAL RECORDS WRITTEN AND ONLY SUMMARY T32           00190000
    RECORDS AS A DEFAULT FOR TSO */                                 00200000
                                                                    00210000
 SUBSYS(STC,NOTYPE(19,40,92),                                       00220001
  EXITS(IEFU29,IEFU83,IEFU84,IEFUTL),                               00230000
  INTERVAL(SMF,SYNC),DETAIL)           /*SP430*/                    00240000
                                                                    00250000
 /* WRITE ALL RECORDS AS BY  SYSTEM DEFAULT, TAKE ONLY THREE        00260000
    EXITS, NOTE: IEFU29 EXECUTES IN THE MASTER ASID WHICH IS A      00270000
    STC ADDRESS SPACE SO IEFU29 MUST BE ON FOR STC. USE ALL OTHER   00280000
    SYS PARAMETERS AS A DEFAULT  */                                 00290000"#;

    let write_dataset = datasets_client
        .write("SYS1.PARMLIB")
        .member("SMFPRM00")
        .if_match("B5C6454F783590AA8EC15BD88E29EA63")
        .text(data)
        .build()
        .await?;

    println!("{:#?}", write_dataset);

    Ok(())
}
