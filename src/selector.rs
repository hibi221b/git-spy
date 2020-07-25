pub const REPO_NAME_AND_HREF_SELECTOR_ARRAY: [&'static str; 10] = [
    REPO_NAME_AND_HREF_SELECTOR_1,
    REPO_NAME_AND_HREF_SELECTOR_2,
    REPO_NAME_AND_HREF_SELECTOR_3,
    REPO_NAME_AND_HREF_SELECTOR_4,
    REPO_NAME_AND_HREF_SELECTOR_5,
    REPO_NAME_AND_HREF_SELECTOR_6,
    REPO_NAME_AND_HREF_SELECTOR_7,
    REPO_NAME_AND_HREF_SELECTOR_8,
    REPO_NAME_AND_HREF_SELECTOR_9,
    REPO_NAME_AND_HREF_SELECTOR_10
];

pub const DESC_SELECTOR_ARRAY: [&'static str; 10] = [
    DESC_SELECTOR_1,
    DESC_SELECTOR_2,
    DESC_SELECTOR_3,
    DESC_SELECTOR_4,
    DESC_SELECTOR_5,
    DESC_SELECTOR_6,
    DESC_SELECTOR_7,
    DESC_SELECTOR_8,
    DESC_SELECTOR_9,
    DESC_SELECTOR_10,
];

pub const STAR_SELECTOR_ARRAY: [&'static str; 10] = [
    STAR_SELECTOR_1,
    STAR_SELECTOR_2,
    STAR_SELECTOR_3,
    STAR_SELECTOR_4,
    STAR_SELECTOR_5,
    STAR_SELECTOR_6,
    STAR_SELECTOR_7,
    STAR_SELECTOR_8,
    STAR_SELECTOR_9,
    STAR_SELECTOR_10,
];

pub const KEYWORDS_SELECTOR_ARRAY: [&'static str; 10] = [
    KEYWORDS_SELECTOR_1,
    KEYWORDS_SELECTOR_2,
    KEYWORDS_SELECTOR_3,
    KEYWORDS_SELECTOR_4,
    KEYWORDS_SELECTOR_5,
    KEYWORDS_SELECTOR_6,
    KEYWORDS_SELECTOR_7,
    KEYWORDS_SELECTOR_8,
    KEYWORDS_SELECTOR_9,
    KEYWORDS_SELECTOR_10,
];

pub const GITHUB_URL                    : &'static str = "https://github.com/";
pub const GITHUB_INPUT_SELECTOR         : &'static str = "body > div.position-relative.js-header-wrapper > header > div > div.HeaderMenu.HeaderMenu--logged-out.position-fixed.top-0.right-0.bottom-0.height-fit.position-lg-relative.d-lg-flex.flex-justify-between.flex-items-center.flex-auto > div.d-lg-flex.flex-items-center.px-3.px-lg-0.text-center.text-lg-left > div > div > div > form > label > input.form-control.input-sm.header-search-input.jump-to-field.js-jump-to-field.js-site-search-focus";
pub const NEXT_PAGE_BUTTON_SELECTOR     : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > div.paginate-container.codesearch-pagination-container > div > a.next_page";
pub const HIT_REPOSITORY_SELECTOR       : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > div.d-flex.flex-column.flex-md-row.flex-justify-between.border-bottom.pb-3.position-relative > h3";
pub const LENGTH_OF_ELEMENTS            : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li";

//1                                                                                                                                                                     //li:nth-child(1~10)
const REPO_NAME_AND_HREF_SELECTOR_1     : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(1) > div.mt-n1 > div.f4.text-normal > a";
const DESC_SELECTOR_1                   : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(1) > div.mt-n1 > p";
const STAR_SELECTOR_1                   : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(1) > div.mt-n1 > div:nth-child(3) > div.d-flex.flex-wrap.text-small.text-gray > div:nth-child(1) > a";
const KEYWORDS_SELECTOR_1               : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(1) > div.mt-n1 > div:nth-child(3) > div:nth-child(1)";

//2
const REPO_NAME_AND_HREF_SELECTOR_2     : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(2) > div.mt-n1 > div.f4.text-normal > a";
const DESC_SELECTOR_2                   : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(2) > div.mt-n1 > p";
const STAR_SELECTOR_2                   : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(2) > div.mt-n1 > div:nth-child(3) > div.d-flex.flex-wrap.text-small.text-gray > div:nth-child(1) > a";
const KEYWORDS_SELECTOR_2               : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(2) > div.mt-n1 > div:nth-child(3) > div:nth-child(1)";

//3
const REPO_NAME_AND_HREF_SELECTOR_3     : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(3) > div.mt-n1 > div.f4.text-normal > a";
const DESC_SELECTOR_3                   : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(3) > div.mt-n1 > p";
const STAR_SELECTOR_3                   : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(3) > div.mt-n1 > div:nth-child(3) > div.d-flex.flex-wrap.text-small.text-gray > div:nth-child(1) > a";
const KEYWORDS_SELECTOR_3               : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(3) > div.mt-n1 > div:nth-child(3) > div:nth-child(1)";

//4
const REPO_NAME_AND_HREF_SELECTOR_4     : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(4) > div.mt-n1 > div.f4.text-normal > a";
const DESC_SELECTOR_4                   : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(4) > div.mt-n1 > p";
const STAR_SELECTOR_4                   : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(4) > div.mt-n1 > div:nth-child(3) > div.d-flex.flex-wrap.text-small.text-gray > div:nth-child(1) > a";
const KEYWORDS_SELECTOR_4               : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(4) > div.mt-n1 > div:nth-child(3) > div:nth-child(1)";

//5
const REPO_NAME_AND_HREF_SELECTOR_5     : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(5) > div.mt-n1 > div.f4.text-normal > a";
const DESC_SELECTOR_5                   : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(5) > div.mt-n1 > p";
const STAR_SELECTOR_5                   : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(5) > div.mt-n1 > div:nth-child(3) > div.d-flex.flex-wrap.text-small.text-gray > div:nth-child(1) > a";
const KEYWORDS_SELECTOR_5               : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(5) > div.mt-n1 > div:nth-child(3) > div:nth-child(1)";

//6
const REPO_NAME_AND_HREF_SELECTOR_6     : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(6) > div.mt-n1 > div.f4.text-normal > a";
const DESC_SELECTOR_6                   : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(6) > div.mt-n1 > p";
const STAR_SELECTOR_6                   : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(6) > div.mt-n1 > div:nth-child(3) > div.d-flex.flex-wrap.text-small.text-gray > div:nth-child(1) > a";
const KEYWORDS_SELECTOR_6               : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(6) > div.mt-n1 > div:nth-child(3) > div:nth-child(1)";

//7
const REPO_NAME_AND_HREF_SELECTOR_7     : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(7) > div.mt-n1 > div.f4.text-normal > a";
const DESC_SELECTOR_7                   : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(7) > div.mt-n1 > p";
const STAR_SELECTOR_7                   : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(7) > div.mt-n1 > div:nth-child(3) > div.d-flex.flex-wrap.text-small.text-gray > div:nth-child(1) > a";
const KEYWORDS_SELECTOR_7               : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(7) > div.mt-n1 > div:nth-child(3) > div:nth-child(1)";

//8
const REPO_NAME_AND_HREF_SELECTOR_8     : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(8) > div.mt-n1 > div.f4.text-normal > a";
const DESC_SELECTOR_8                   : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(8) > div.mt-n1 > p";
const STAR_SELECTOR_8                   : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(8) > div.mt-n1 > div:nth-child(3) > div.d-flex.flex-wrap.text-small.text-gray > div:nth-child(1) > a";
const KEYWORDS_SELECTOR_8               : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(8) > div.mt-n1 > div:nth-child(3) > div:nth-child(1)";

//9
const REPO_NAME_AND_HREF_SELECTOR_9     : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(9) > div.mt-n1 > div.f4.text-normal > a";
const DESC_SELECTOR_9                   : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(9) > div.mt-n1 > p";
const STAR_SELECTOR_9                   : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(9) > div.mt-n1 > div:nth-child(3) > div.d-flex.flex-wrap.text-small.text-gray > div:nth-child(1) > a";
const KEYWORDS_SELECTOR_9               : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(9) > div.mt-n1 > div:nth-child(3) > div:nth-child(1)";

//10
const REPO_NAME_AND_HREF_SELECTOR_10    : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(10) > div.mt-n1 > div.f4.text-normal > a";
const DESC_SELECTOR_10                  : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(10) > div.mt-n1 > p";
const STAR_SELECTOR_10                  : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(10) > div.mt-n1 > div:nth-child(3) > div.d-flex.flex-wrap.text-small.text-gray > div:nth-child(1) > a";
const KEYWORDS_SELECTOR_10              : &'static str = "#js-pjax-container > div > div.col-12.col-md-9.float-left.px-2.pt-3.pt-md-0.codesearch-results > div > ul > li:nth-child(10) > div.mt-n1 > div:nth-child(3) > div:nth-child(1)";