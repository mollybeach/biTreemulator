//write a binary tree in rust and then write a function to print it out in a
//bfs order
//this is a binary tree
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            value: value,
            left: None,
            right: None,
        }
    }
}

fn main() {
    let mut root = Node::new(1);
    root.left = Some(Box::new(Node::new(2)));
    root.right = Some(Box::new(Node::new(3)));
    root.left.as_mut().unwrap().left = Some(Box::new(Node::new(4)));
    root.left.as_mut().unwrap().right = Some(Box::new(Node::new(5)));
    root.right.as_mut().unwrap().left = Some(Box::new(Node::new(6)));
    root.right.as_mut().unwrap().right = Some(Box::new(Node::new(7)));
    root.left.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(Node::new(8)));
    root.left.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(Node::new(9)));
    root.left.as_mut().unwrap().right.as_mut().unwrap().left = Some(Box::new(Node::new(10)));
    root.left.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(Node::new(11)));
    root.right.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(Node::new(12)));
    root.right.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(Node::new(13)));
    root.right.as_mut().unwrap().right.as_mut().unwrap().left = Some(Box::new(Node::new(14)));
    root.right.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(Node::new(15)));
    root.left.as_mut().unwrap().left.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(Node::new(16)));
    root.left.as_mut().unwrap().left.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(Node::new(17)));
    root.left.as_mut().unwrap().left.as_mut().unwrap().right.as_mut().unwrap().left = Some(Box::new(Node::new(18)));
    root.left.as_mut().unwrap().left.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(Node::new(19)));
    root.left.as_mut().unwrap().right.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(Node::new(20)));
    root.left.as_mut().unwrap().right.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(Node::new(21)));
    root.left.as_mut().unwrap().right.as_mut().unwrap().right.as_mut().unwrap().left = Some(Box::new(Node::new(22)));
    root.left.as_mut().unwrap().right.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(Node::new(23)));
    root.right.as_mut().unwrap().left.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(Node::new(24)));
    root.right.as_mut().unwrap().left.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(Node::new(25)));
}

//this is a function to print out the tree in a bfs order
fn print_tree(root: &Node) {
    let mut queue = Vec::new();
    queue.push(root);
    while !queue.is_empty() {
        let node = queue.remove(0);
        println!("{}", node.value);
        if node.left.is_some() {
            queue.push(node.left.as_ref().unwrap());
        }
        if node.right.is_some() {
            queue.push(node.right.as_ref().unwrap());
        }
    }
}

//this is a function to print out the tree in a dfs order
fn print_tree_dfs(root: &Node) {
    println!("{}", root.value);
    if root.left.is_some() {
        print_tree_dfs(root.left.as_ref().unwrap());
    }
    if root.right.is_some() {
        print_tree_dfs(root.right.as_ref().unwrap());
    }
}

//this is a function to print out the tree in a dfs order
fn print_tree_dfs_pre(root: &Node) {
    if root.left.is_some() {
        print_tree_dfs(root.left.as_ref().unwrap());
    }
    println!("{}", root.value);
    if root.right.is_some() {
        print_tree_dfs(root.right.as_ref().unwrap());
    }
}

//this is a function to print out the tree in a dfs order
fn print_tree_dfs_post(root: &Node) {
    if root.left.is_some() {
        print_tree_dfs(root.left.as_ref().unwrap());
    }
    if root.right.is_some() {
        print_tree_dfs(root.right.as_ref().unwrap());
    }
    println!("{}", root.value);
}

//this is a function to print out the tree in a dfs order
fn print_tree_dfs_in(root: &Node) {
    if root.left.is_some() {
        print_tree_dfs(root.left.as_ref().unwrap());
    }
    println!("{}", root.value);
    if root.right.is_some() {
        print_tree_dfs(root.right.as_ref().unwrap());
    }
}







/*************************************TEXT STYLES************************************/
@use "./colors" as *;
@use "./breakpoints" as *;
@use "./variables" as *;
/*****************************************H1-Main-heading***************************/

@mixin h1-main-header {
    font-family: AvenirNextLTProBold;
    font-size: 1.5rem; //24px 
    line-height: 2.0rem; //32px
    color: $white;
    @include tablet {
        font-size: 2.3rem; //36px 2.25rem 
        line-height: 2.8rem; //44px
    }
    @include desktop {
        font-size: 2.3rem; //36px 2.25rem 
        line-height: 2.8rem; //44px
    }
}

@mixin h1-main-header-regular {
    @include h1-main-header;
    font-family: AvenirNextLTProRegular;
}

@mixin h1-main-header-black {
    @include h1-main-header;
    color: $black;
}

@mixin h1-desktop-size-that-should-be-available-on-mobile {
    color: $white;
    font-family: AvenirNextLTProBold;
    font-size: 8.0vw;
    line-height: 10.8vw;
}


/*********************H2 - Header subheader for mobile*********************/

@mixin h2-header {
    font-size: 0.9rem; //14px 
    line-height: 1.4rem; //22px 
    font-family: AvenirNextLTProDemi;
    color: $black;
    @include tablet {
        font-size: 1.5rem; //24px 1.5rem 
        line-height: 2.0; //32px 2rem
        font-family: AvenirNextLTProBold;
    }
    @include desktop {
        font-size: 1.5rem; //24px 1.5rem 
        line-height: 2.0rem; //32px 2rem 
    }
}

@mixin h2-header-white {
    @include h2-header;
    color: $white;
}

@mixin h2-header-responsive-white {
    @include h2-header-responsive;
    color: $white;
}

@mixin h2-desktop-size-that-should-be-available-on-mobile {
    color: $white;
    font-size: 1.5rem; //24px 1.5rem 
    line-height: 2.0; //32px 2rem
    font-family: AvenirNextLTProBold;
}


/*******************************H3 - Subheader for Tablet and Desktop************/

@mixin h3-subheader {
    font-size: 1.0rem; //16px 
    line-height: 1.5rem; // 24px
    font-family: AvenirNextLTProDemi;
    color: $black;
    @include tablet {
        font-size: 1.0rem; //16px 
        line-height: 1.5rem; // 24px
    }
    @include desktop {
        font-size: 1.0rem; //16px 
        line-height: 1.5rem; // 24px
    }
}

@mixin h3-subheader-white {
    @include h3-subheader;
    color: $white;
}

@mixin h3-subheader-bold {
    @include h3-subheader;
    font-family: AvenirNextLTProBold;
}

@mixin h3-subheader-white-bold {
    @include h3-subheader;
    font-family: AvenirNextLTProBold;
    color: $white;
}


/********************H4 Paragraph & Comments******************************/

@mixin h4-paragraphs-and-comments {
    font-family: AvenirNextLTProRegular;
    color: $black;
    font-size: 0.8rem; // 13px
    line-height: 1.1rem; //18px
    @include tablet {
        font-size: 0.9rem; // 14px
        line-height: 1.4rem; //22px
    }
    @include desktop {
        font-size: 0.9rem; // 14px
        line-height: 1.4rem; // 22px 
    }
}

@mixin h4-paragraphs-and-comments-bold {
    @include h4-paragraphs-and-comments;
    font-family: AvenirNextLTProBold;
}

@mixin h4-paragraphs-and-comments-white {
    @include h4-paragraphs-and-comments;
    color: $white;
}

@mixin h4-paragraphs-and-comments-white-bold {
    @include h4-paragraphs-and-comments-bold;
    color: $white;
}

@mixin h4-paragraphs-and-comments-white-demi {
    @include h4-paragraphs-and-comments-white;
    font-family: AvenirNextLTProDemi;
    color: $white;
}

$time: 330ms;
$btns: ( 
alpha: ( gstart: #21D4FD,
gend: #B721FF,
tstart: #B721FF,
tend: #21D4FD),
beta: ( gstart: #08AEEA,
gend: #2AF598,
tstart: #2AF598,
tend: #08AEEA),
gamma: ( gstart: #FEE140,
gend: #FA709A,
tstart: #FA709A,
tend: #FEE140),
delta: ( gstart: #3EECAC,
gend: #EE74E1,
tstart: #EE74E1,
tend: #3EECAC));
.btn {
	outline: none;;
	margin-right: 30px;
	margin-top: 30px;
	font-family: 'Source Sans Pro', sans-serif;
	font-weight: 900;
	padding: 1.25rem 2rem;
	font-size: 1rem;
	border-radius: 3.5rem / 100%;
	position: relative;
	min-width: 15rem;
	max-width: 90vw;
	overflow: hidden;
	border: 0;
	cursor: pointer;
	text-transform: uppercase;
	letter-spacing: 0.05em;
	transition: all $time;
	outline: none;;
	&+& {
		outline: none;;
		margin-top: 1rem;
	}
	span {
		outline: none;;
		position: relative;
		z-index: 1;
	}
	&:before {
		outline: none;;
		content: "";
		background-color: #21D4FD;
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		transform: scale(4) translateX(-100%);
		transition: all $time * 1.5 ease-out;
		outline: none;;
	}
	&:hover,
	&:focus,
	&:active {
		&:before {
			outline: none;;
			transform: scale(4) translate(37%);
		}
	}
}

@each $bname,
$bcolors in $btns {
	.btn--#{$bname} {
		outline: none;;
		background-color: map-get($bcolors, gstart);
		color: map-get($bcolors, tstart);
		&:before {
			background-color: map-get($bcolors, gend);
			background-image: linear-gradient(to right, map-get($bcolors, gend) 30%, map-get($bcolors, gstart) 100%);
			position: absolute;
		}
		&:hover,
		&:focus,
		&:active {
			outline: none;;
			color: map-get($bcolors, tend);
		}
	}
}

.btn-form {
	outline: none;;
	outline: none;;
	margin-right: 30px;
	margin-top: 30px;
	font-family: 'Source Sans Pro', sans-serif;
	font-weight: 900;
	padding: 1.25rem 2rem;
	font-size: 1rem;
	border-radius: 3.5rem / 100%;
	position: relative;
	min-width: 15rem;
	max-width: 90vw;
	overflow: hidden;
	border: 0;
	cursor: pointer;
	text-transform: uppercase;
	letter-spacing: 0.05em;
	transition: all $time;
	 ;
	&+& {
		margin-top: 1rem;
		 ;
	}
	span {
		position: relative;
		z-index: 1;
		 ;
	}
	&:before {
		content: "";
		background-color: #21D4FD;
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		transform: scale(4) translateX(-100%);
		transition: all $time * 1.5 ease-out;
		 ;
	}
	&:hover,
	&:focus,
	&:active {
		&:before {
			 
			//transform: scale(4) translate(37%);
		  //  color: map-get($bcolors, tstart);
		  background-color:#B721FF;;
		}
	}
}

@each $bname,
$bcolors in $btns {
	.btn-form--#{$bname} {
		background-color: map-get($bcolors, gstart);
		color: map-get($bcolors, tstart);
		&:before {
			background-color: map-get($bcolors, gend);
			background-image: linear-gradient(to right, map-get($bcolors, gend) 30%, map-get($bcolors, gstart) 100%);
			position: absolute;
		}
		&:hover,
		&:focus,
		&:active {
		//	color: map-get($bcolors, tstart);
		color: #B721FF;
		}
	}
}
/********************************h4-lists-and-contacts-information*************/

@mixin h4-list-contact-information {
    color: $black;
    font-family: AvenirNextLTProRegular;
    font-size: 0.8rem; //13px 
    line-height: 1.1rem; //18px 
    @include tablet {
        font-size: 0.9rem; //14px 
        line-height: 1.3rem; //20px
    }
    @include desktop {
        font-size: 0.9rem; //14px 
        line-height: 1.3rem; //20px 
    }
}

@mixin h4-list-contact-information-bold {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProBold;
}

@mixin h4-list-contact-information-demi {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProDemi;
}

@mixin h4-list-contact-information-white {
    @include h4-list-contact-information;
    color: $white;
}

@mixin h4-list-contact-information-white-demi {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProDemi;
    color: $white;
}

@mixin h4-list-contact-information-white-bold {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProBold;
    color: $white;
}


//********************************P-Paragraph****************************/

@mixin p-paragraph {
    font-family: AvenirNextLTProRegular;
    color: $black;
    font-size: 0.8rem; //13px
    line-height: 1.1rem; //18px
    @include tablet {
        font-size: 0.9rem; //13px 
        line-height: 1.3rem; //18px 
        display: inline-block;
    }
    @include desktop {
        font-size: 0.9rem; //13px 
        line-height: 1.3rem; //18px
        display: inline-block;
    }
}

@mixin p-paragraph-bold {
    @include p-paragraph;
    font-family: AvenirNextLTProBold;
}

@mixin p-paragraph-white {
    @include p-paragraph;
    color: white;
}

@mixin p-paragraph-white-bold {
    @include p-paragraph-bold;
    color: white;
}

@mixin h1-page-header {
	font-family: TitilliumWebSemiBold600;
	color: $black;
	font-size: 28px; //28px
	line-height: 36px; //36px
	@include tablet {
        font-size: 32px; //32px
        line-height: 40px; //40px
	}
	@include desktop {
        font-size: 32px; //32px
        line-height: 40px; //40px
	}
}

@mixin h2-subheader {
	font-family: TitilliumWebSemiBold600;
	color: $black;
	font-size: 20px; //20px
	line-height: 28px; //28px
	@include tablet {
        font-size: 24px; //24px
        line-height: 32px; //28px
	}
	@include desktop {
        font-size: 24px; //24px
        line-height: 32px; //28px
	}
}
@mixin h3-labels-links-buttons {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 13px; //13px
	line-height: 20px; //20px
	@include tablet {
        font-size: 14px; //14px
        line-height: 22px; //22px
	}
	@include desktop {
        font-size: 14px; //14px
        line-height: 22px; //22px
	}
}
@mixin h4-labels-table-header {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 11px; //11px
	line-height: 16px; //16px
	@include tablet {
        font-size: 12px; //12px
        line-height: 18px; //18px
	}
	@include desktop {
        font-size: 12px; //12px
        line-height: 18px; //18px
	}
}
@mixin p1-body-large {
	font-family: TitilliumWebRegular400;
	color: $black;
    font-size: 15px; //15px
    line-height: 26px; //26px
	@include tablet {
        font-size: 16px; //12px
        line-height: 28px; //18px
	}
	@include desktop {
        font-size: 16px; //12px
        line-height: 28px; //18px
	}
}
@mixin p2-body-medium {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 16px; //13px
	line-height: 28px; //20px
	@include tablet {
        font-size: 13px; //13px
        line-height: 20px; //20px
	}
	@include desktop {
        font-size: 13px; //13px
        line-height: 20px; //20px
	}
}
@mixin p2-body-small {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 11px; //13px
	line-height: 16px; //20px
	@include tablet {
        font-size: 12px; //13px
        line-height: 18px; //20px
	}
	@include desktop {
        font-size: 12px; //13px
        line-height: 18px; //20px
	}
}

/*******************************************H5-Labels***********************/

@mixin h5-labels {
    font-size: 0.8rem; //12px
    line-height: 1.3rem; //20px 
    font-family: AvenirNextLTProRegular;
    color: $silver; //labels
    @include tablet {
        font-size: 0.8rem; //12px
        line-height: 1.3rem; // 20px
        font-family: AvenirNextLTProDemi;
    }
    @include desktop {
        font-size: 0.8rem; //12px
        line-height: 1.8rem; // 20px
    }
}

@mixin h5-labels-demi {
    @include h5-labels;
    font-family: AvenirNextLTProDemi;
}

@mixin h5-labels-bold {
    @include h5-labels;
    font-family: AvenirNextLTProBold;
}


//Breakpoints
@mixin tablet {
    @media (min-width: $breakpoint-tablet) {
        @content;
    }
}

@mixin desktop {
    @media (min-width: $breakpoint-desktop) {
        @content;
    }
}

//Text Styles- Desktop

@mixin d-header {
    font-family: $font-family;
    font-size: 1.875rem;
    line-height: 2.25rem;
    font-weight: 700;
	color: $text-black;
}

@mixin d-subheader {
	font-family: $font-family;
    font-size: 1.875rem;
    line-height: 2.25rem;
    font-weight: 500;
	color: $text-black;
}

@mixin d-subheader-grey {
	font-family: $font-family;
    font-size: 1.75rem;
    line-height: 2rem;
	color: $text-grey;
    font-weight: 500;
}

@mixin d-paragraph-bold {
	font-family: $font-family;
    font-size: 1.125rem;
    line-height: 1.25rem;
	color: $text-black;
    font-weight: 700;
}

@mixin d-paragraph-large {
	font-family: $font-family;
    font-size: 1.375rem;
    line-height: 1.4rem;
	color: $text-grey;
    font-weight: 400;
}

@mixin d-paragraph-medium {
	font-family: $font-family;
    font-size: 1.25rem;
    line-height: 1.4rem;
	color: $text-grey;
    font-weight: 400;
}

@mixin d-paragraph-small {
	font-family: $font-family;
    font-size: 1.4rem;
    line-height: 1rem;
	color: $text-grey;
    font-weight: 400;
}

//Button Styling
@mixin button-blue {
    background-color: $secondary-color-indigo;
    border-radius: 5px;
    color: white;
    padding: 0.875rem 1.813rem;
    appearance: none;
    border: none;
}

//Component Styling

@mixin component-styling {
    box-shadow: 0px 10px 25px rgba(19, 24, 44, 0.1);
    border-radius: 10px;
    padding: 3.75rem;
}
/***************************************STANDARD PADDING**************************************************/
@mixin standard-padding-right {
	padding-right: $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding-right: $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding-right: $mockup-column-outside-desktop; //80px right 80px left
	}
  }

  @mixin standard-padding-left {
	padding-left: $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding-left: $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding-left: $mockup-column-outside-desktop; //80px right 80px left
	}
  }
  
  @mixin standard-padding-horizontal {
	padding: 0 $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding: 0 $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding: 0 $mockup-column-outside-desktop; //80px right 80px left
	}
  }
  
  
  /**********************************WIDTH*100%*PADDING**********************************************/
  
  @mixin width-100-percent-minus-standard-padding-horizontal {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent} * 2));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent} * 2));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent} * 2));
	}
  }
  
  @mixin width-100-percent-minus-standard-padding-left {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent}));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent}));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent}));
	}
  }
  
  @mixin width-100-percent-minus-standard-padding-right {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent}));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent}));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent}));
	}
  }
  

  /*******************WIDTH MINUS MOCKUP COLUMN************/
  @mixin width-100-percent-minus-mockup-column-outside {
	width: calc(100% - #{$mockup-column-outside-mobile});
	@include tablet {
	  width: calc(100% - #{$mockup-column-outside-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-outside-desktop});
	}
  }
  @mixin width-100-percent-minus-mockup-column-outside-horizontal {
	width: calc(100% - (#{$mockup-column-outside-mobile} * 2));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet} *2));
	}
	@include desktop {
	  width: calc(100% - (#{mockup-column-outside-desktop} *2));
	}
  }
  @mixin width-100-percent-minus-mockup-column-small {
	width: calc(100% - #{mockup-column-small-mobile});
	@include tablet {
	  width: calc(100% -#{mockup-column-small-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-small-desktop});
	}
  }
  
  @mixin width-100-percent-minus-mockup-column-large {
	width: calc(100% - #{mockup-column-large-mobile});
	@include tablet {
	  width: calc(100% -#{mockup-column-large-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-large-desktop});
	}
  }
  



@mixin fancybutton {
  font-family: 'Source Sans Pro', sans-serif;
  font-weight: 900;
  padding: 1.25rem 2rem;
  font-size: 1rem;
  border-radius: 3.5rem / 100%;
  position: relative;
  min-width: 15rem;
  max-width: 90vw;
  overflow: hidden;
  border: 0;
  cursor: pointer;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  transition: all $time;
  
  & + & {
    margin-top: 1rem;
  }
  
  span {
    position: relative;
    z-index: 1;
  }
  
  &:before {
    content: "";
    background-color: #21D4FD;
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    transform: scale(4) translateX(-100%);
    transition: all $time * 1.5 ease-out;
  }
  
  &:hover,
  &:focus,
  &:active {    
    &:before {
      transform: scale(4) translate(37%);
    } 
  }
}


  /**********************************************DIVIDERS******************************************************/
  
  @mixin divider {
	background: $silver;
	width: 100%;
	height: 1px;
  }
  
  @mixin divider-form {
	background: $silver;
	width: 100%;
	height: 1px;
	margin-bottom: $increment2-px;
  }
  
  /**********************************************LOGOS******************************************************/
  
  @mixin logo-brainflix {
	img {
	  width: 120px; //120px
	  height: 24px; //24px
	  margin-left: 1rem;
	  @include tablet {
		margin-left: 2rem;
		width: 120px; //120px
		height: 24px; //24px
	  }
	  @include desktop {
		margin-left: 5rem;
	  }
	}
  }
  
  /**********************************************BUTTONS******************************************************/
  
  @mixin button {
	background-color: $brainflix-blue;
	@include h4-paragraphs-and-comments-white-demi;
	&:hover {
	  background: $brainflix-dark-blue-hover;
	}
	&:focus {
	  outline: none;
	}
	width: $increment28-px; //226px
	height: $increment6-px; 
	border-radius: $increment0-px; 
	border-color: $brainflix-blue;
	@include tablet {
	  padding: 0;
	  width: $increment20-px; //160px
	  height: $increment6-px; 
	}
  }
  
  /**********************************************MOCKUP COLUMNS****************************************************************/
  
  /*******************RIGHT*****************/
  
  @mixin mockup-column-outside-right {
	margin-right: $mockup-column-outside-mobile;
	@include tablet {
	  margin-right: $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-right {
	margin-right: $mockup-column-small-mobile;
	@include tablet {
	  margin-right: $mockup-column-small-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-right {
	margin-right: $mockup-column-large-mobile;
	@include tablet {
	  margin-right: $mockup-column-large-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-large-desktop;
	}
  }
  
  /*******************LEFT***************/
  
  @mixin mockup-column-outside-left {
	margin-left: $mockup-column-outside-mobile;
	@include tablet {
	  margin-left: $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-left {
	margin-left: $mockup-column-small-mobile;
	@include tablet {
	  margin-left: $mockup-column-small-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-left {
	margin-left: $mockup-column-large-mobile;
	@include tablet {
	  margin-left: $mockup-column-large-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-large-desktop;
	}
  }
  
  /*********************HORIZONTAL**************/
  
  @mixin mockup-column-outside-horizontal {
	margin: 0 $mockup-column-outside-mobile;
	@include tablet {
	  margin: 0 $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-horizontal {
	margin: 0 $mockup-column-small-mobile;
	@include tablet {
	  margin: 0 $mockup-column-small-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-horizontal {
	margin: 0 $mockup-column-large-mobile;
	@include tablet {
	  margin: 0 $mockup-column-large-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-large-desktop;
	}
  }
  
  /*********************************************WIDTH 100% MINUS INCREMENTS*********************************/
  
  @mixin width-100-percent-minus-increment0 {
	width: calc(100% - #{increment0-mobile});
	@include tablet {
	  width: calc(100% -#{increment0-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment0-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment1 {
	width: calc(100% - #{increment1-mobile});
	@include tablet {
	  width: calc(100% - #{increment1-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment1-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment2 {
	width: calc(100% - #{increment2-mobile});
	@include tablet {
	  width: calc(100% -#{increment2-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment2-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment3 {
	width: calc(100% - #{increment3-mobile});
	@include tablet {
	  width: calc(100% -#{increment3-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment3-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment4 {
	width: calc(100% - #{increment4-mobile});
	@include tablet {
	  width: calc(100% -#{increment4-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment4-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment5 {
	width: calc(100% - #{increment5-mobile});
	@include tablet {
	  width: calc(100% -#{increment5-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment5-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment6 {
	width: calc(100% - #{increment6-mobile});
	@include tablet {
	  width: calc(100% -#{increment6-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment6-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment7 {
	width: calc(100% - #{increment7-mobile});
	@include tablet {
	  width: calc(100% -#{increment7-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment7-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment8 {
	width: calc(100% - #{increment8-mobile});
	@include tablet {
	  width: calc(100% -#{increment8-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment8-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment9 {
	width: calc(100% - #{increment9-mobile});
	@include tablet {
	  width: calc(100% -#{increment9-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment9-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment10 {
	width: calc(100% - #{increment10-mobile});
	@include tablet {
	  width: calc(100% -#{increment10-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment10-desktop});
	}
  }
  

  
  /************************************%%%%%%%%%%%%%%%%%%%%%******************************************************
  /******%%%%%%%%%%%%%%%%%%%%%%%****CONTENTS LIST:*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****TOP MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *****%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%******RIGHT MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***BOTTOM MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***LEFT MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ***********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*VERTICAL MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ***********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*HORIZTONAL**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***WIDTH **********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%****HEIGHT**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****OTHER COMMON INCREMENTS*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
   ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****MOCKUP COLUMN PADDING*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********/
  
  /*******************************************************INCREMENT MARGIN TOP******************************************/
  
  @mixin increment-0-top {
	margin-top: $increment0-mobile;
	@include tablet {
	  margin-top: $increment0-tablet;
	}
	@include desktop {
	  margin-top: $increment0-desktop;
	}
  }
  
  @mixin increment-1-top {
	margin-top: $increment1-mobile;
	@include tablet {
	  margin-top: $increment1-tablet;
	}
	@include desktop {
	  margin-top: $increment1-desktop;
	}
  }
  
  @mixin increment-2-top {
	margin-top: $increment2-mobile;
	@include tablet {
	  margin-top: $increment2-tablet;
	}
	@include desktop {
	  margin-top: $increment2-desktop;
	}
  }
  
  @mixin increment-3-top {
	margin-top: $increment3-mobile;
	@include tablet {
	  margin-top: $increment3-tablet;
	}
	@include desktop {
	  margin-top: $increment3-desktop;
	}
  }
  
  @mixin increment-4-top {
	margin-top: $increment4-mobile;
	@include tablet {
	  margin-top: $increment4-tablet;
	}
	@include desktop {
	  margin-top: $increment4-desktop;
	}
  }
  
  @mixin increment-5-top {
	margin-top: $increment5-mobile;
	@include tablet {
	  margin-top: $increment5-tablet;
	}
	@include desktop {
	  margin-top: $increment5-desktop;
	}
  }
  
  @mixin increment-6-top {
	margin-top: $increment6-mobile;
	@include tablet {
	  margin-top: $increment6-tablet;
	}
	@include desktop {
	  margin-top: $increment6-desktop;
	}
  }
  
  @mixin increment-7-top {
	margin-top: $increment7-mobile;
	@include tablet {
	  margin-top: $increment7-tablet;
	}
	@include desktop {
	  margin-top: $increment7-desktop;
	}
  }
  
  @mixin increment-8-top {
	margin-top: $increment8-mobile;
	@include tablet {
	  margin-top: $increment8-tablet;
	}
	@include desktop {
	  margin-top: $increment8--desktop;
	}
  }
  
  @mixin increment-9-top {
	margin-top: $increment9-mobile;
	@include tablet {
	  margin-top: $increment9-tablet;
	}
	@include desktop {
	  margin-top: $increment9--desktop;
	}
  }
  
  @mixin increment-10-top {
	margin-top: $increment10-mobile;
	@include tablet {
	  margin-top: $increment10-tablet;
	}
	@include desktop {
	  margin-top: $increment10--desktop;
	}
  }
  
  /***********************************INCREMENT RIGHT*************************************************/
  
  @mixin increment-0-right {
	margin-right: $increment0-mobile;
	@include tablet {
	  margin-right: $increment0-tablet;
	}
	@include desktop {
	  margin-right: $increment0-desktop;
	}
  }
  
  @mixin increment-1-right {
	margin-right: $increment1-mobile;
	@include tablet {
	  margin-right: $increment1-tablet;
	}
	@include desktop {
	  margin-right: $increment1-desktop;
	}
  }
  
  @mixin increment-2-right {
	margin-right: $increment2-mobile;
	@include tablet {
	  margin-right: $increment2-tablet;
	}
	@include desktop {
	  margin-right: $increment2-desktop;
	}
  }
  
  @mixin increment-3-right {
	margin-right: $increment3-mobile;
	@include tablet {
	  margin-right: $increment3-tablet;
	}
	@include desktop {
	  margin-right: $increment3-desktop;
	}
  }
  
  @mixin increment-4-right {
	margin-right: $increment4-mobile;
	@include tablet {
	  margin-right: $increment4-tablet;
	}
	@include desktop {
	  margin-right: $increment4-desktop;
	}
  }
  
  @mixin increment-5-right {
	margin-right: $increment5-mobile;
	@include tablet {
	  margin-right: $increment5-tablet;
	}
	@include desktop {
	  margin-right: $increment5-desktop;
	}
  }
  
  @mixin increment-6-right {
	margin-right: $increment6-mobile;
	@include tablet {
	  margin-right: $increment6-tablet;
	}
	@include desktop {
	  margin-right: $increment6-desktop;
	}
  }
  
  @mixin increment-7-right {
	margin-right: $increment7-mobile;
	@include tablet {
	  margin-right: $increment7-tablet;
	}
	@include desktop {
	  margin-right: $increment7-desktop;
	}
  }
  
  @mixin increment-8-right {
	margin-right: $increment8-mobile;
	@include tablet {
	  margin-right: $increment8-tablet;
	}
	@include desktop {
	  margin-right: $increment8--desktop;
	}
  }
  
  @mixin increment-9-right {
	margin-right: $increment9-mobile;
	@include tablet {
	  margin-right: $increment9-tablet;
	}
	@include desktop {
	  margin-right: $increment9--desktop;
	}
  }
  
  @mixin increment-10-right {
	margin-right: $increment10-mobile;
	@include tablet {
	  margin-right: $increment10-tablet;
	}
	@include desktop {
	  margin-right: $increment10--desktop;
	}
  }
  
  /*******************************************************INCREMENT MARGIN BOTTOM******************************************/
  
  @mixin increment-0-bottom {
	margin-bottom: $increment0-mobile;
	@include tablet {
	  margin-bottom: $increment0-tablet;
	}
	@include desktop {
	  margin-bottom: $increment0-desktop;
	}
  }
  
  @mixin increment-1-bottom {
	margin-bottom: $increment1-mobile;
	@include tablet {
	  margin-bottom: $increment1-tablet;
	}
	@include desktop {
	  margin-bottom: $increment1-desktop;
	}
  }
  
  @mixin increment-2-bottom {
	margin-bottom: $increment2-mobile;
	@include tablet {
	  margin-bottom: $increment2-tablet;
	}
	@include desktop {
	  margin-bottom: $increment2-desktop;
	}
  }
  
  @mixin increment-3-bottom {
	margin-bottom: $increment3-mobile;
	@include tablet {
	  margin-bottom: $increment3-tablet;
	}
	@include desktop {
	  margin-bottom: $increment3-desktop;
	}
  }
  
  @mixin increment-4-bottom {
	margin-bottom: $increment4-mobile;
	@include tablet {
	  margin-bottom: $increment4-tablet;
	}
	@include desktop {
	  margin-bottom: $increment4-desktop;
	}
  }
  
  @mixin increment-5-bottom {
	margin-bottom: $increment5-mobile;
	@include tablet {
	  margin-bottom: $increment5-tablet;
	}
	@include desktop {
	  margin-bottom: $increment5-desktop;
	}
  }
  
  @mixin increment-6-bottom {
	margin-bottom: $increment6-mobile;
	@include tablet {
	  margin-bottom: $increment6-tablet;
	}
	@include desktop {
	  margin-bottom: $increment6-desktop;
	}
  }
  
  @mixin increment-7-bottom {
	margin-bottom: $increment7-mobile;
	@include tablet {
	  margin-bottom: $increment7-tablet;
	}
	@include desktop {
	  margin-bottom: $increment7-desktop;
	}
  }
  
  @mixin increment-8-bottom {
	margin-bottom: $increment8-mobile;
	@include tablet {
	  margin-bottom: $increment8-tablet;
	}
	@include desktop {
	  margin-bottom: $increment8--desktop;
	}
  }
  
  @mixin increment-9-bottom {
	margin-bottom: $increment9-mobile;
	@include tablet {
	  margin-bottom: $increment9-tablet;
	}
	@include desktop {
	  margin-bottom: $increment9--desktop;
	}
  }
  
  @mixin increment-10-bottom {
	margin-bottom: $increment10-mobile;
	@include tablet {
	  margin-bottom: $increment10-tablet;
	}
	@include desktop {
	  margin-bottom: $increment10--desktop;
	}
  }
  
  /*******************************************************INCREMENT LEFT******************************************/
  
  @mixin increment-0-left {
	margin-left: $increment0-mobile;
	@include tablet {
	  margin-left: $increment0-tablet;
	}
	@include desktop {
	  margin-left: $increment0-desktop;
	}
  }
  
  @mixin increment-1-left {
	margin-left: $increment1-mobile;
	@include tablet {
	  margin-left: $increment1-tablet;
	}
	@include desktop {
	  margin-left: $increment1-desktop;
	}
  }
  
  @mixin increment-2-left {
	margin-left: $increment2-mobile;
	@include tablet {
	  margin-left: $increment2-tablet;
	}
	@include desktop {
	  margin-left: $increment2-desktop;
	}
  }
  
  @mixin increment-3-left {
	margin-left: $increment3-mobile;
	@include tablet {
	  margin-left: $increment3-tablet;
	}
	@include desktop {
	  margin-left: $increment3-desktop;
	}
  }
  
  @mixin increment-4-left {
	margin-left: $increment4-mobile;
	@include tablet {
	  margin-left: $increment4-tablet;
	}
	@include desktop {
	  margin-left: $increment4-desktop;
	}
  }
  
  @mixin increment-5-left {
	margin-left: $increment5-mobile;
	@include tablet {
	  margin-left: $increment5-tablet;
	}
	@include desktop {
	  margin-left: $increment5-desktop;
	}
  }
  
  @mixin increment-6-left {
	margin-left: $increment6-mobile;
	@include tablet {
	  margin-left: $increment6-tablet;
	}
	@include desktop {
	  margin-left: $increment6-desktop;
	}
  }
  
  @mixin increment-7-left {
	margin-left: $increment7-mobile;
	@include tablet {
	  margin-left: $increment7-tablet;
	}
	@include desktop {
	  margin-left: $increment7-desktop;
	}
  }
  
  @mixin increment-8-left {
	margin-left: $increment8-mobile;
	@include tablet {
	  margin-left: $increment8-tablet;
	}
	@include desktop {
	  margin-left: $increment8--desktop;
	}
  }
  
  @mixin increment-9-left {
	margin-left: $increment9-mobile;
	@include tablet {
	  margin-left: $increment9-tablet;
	}
	@include desktop {
	  margin-left: $increment9--desktop;
	}
  }
  
  @mixin increment-10-left {
	margin-left: $increment10-mobile;
	@include tablet {
	  margin-left: $increment10-tablet;
	}
	@include desktop {
	  margin-left: $increment10--desktop;
	}
  }
  
  /************************************VERTICAL MARGIN INCREMENTS******************************************************/
  
  @mixin increment-0-vertical {
	margin: $increment0-mobile 0;
	@include tablet {
	  margin: $increment0-tablet 0;
	}
	@include deskvertical {
	  margin: $increment0-desktop 0;
	}
  }
  
  @mixin increment-1-vertical {
	margin: $increment1-mobile 0;
	@include tablet {
	  margin: $increment1-tablet 0;
	}
	@include desktop {
	  margin: $increment1-desktop 0;
	}
  }
  
  @mixin increment-2-vertical {
	margin: $increment2-mobile 0;
	@include tablet {
	  margin: $increment2-tablet 0;
	}
	@include desktop {
	  margin: $increment2-desktop 0;
	}
  }
  
  @mixin increment-3-vertical {
	margin: $increment3-mobile 0;
	@include tablet {
	  margin: $increment3-tablet 0;
	}
	@include desktop {
	  margin: $increment3-desktop 0;
	}
  }
  
  @mixin increment-4-vertical {
	margin: $increment4-mobile 0;
	@include tablet {
	  margin: $increment4-tablet 0;
	}
	@include desktop {
	  margin: $increment4-desktop 0;
	}
  }
  
  @mixin increment-5-vertical {
	margin: $increment5-mobile 0;
	@include tablet {
	  margin: $increment5-tablet 0;
	}
	@include desktop {
	  margin: $increment5-desktop 0;
	}
  }
  
  @mixin increment-6-vertical {
	margin: $increment6-mobile 0;
	@include tablet {
	  margin: $increment6-tablet 0;
	}
	@include desktop {
	  margin: $increment6-desktop 0;
	}
  }
  
  @mixin increment-7-vertical {
	margin: $increment7-mobile 0;
	@include tablet {
	  margin: $increment7-tablet 0;
	}
	@include desktop {
	  margin: $increment7-desktop 0;
	}
  }
  
  @mixin increment-8-vertical {
	margin: $increment8-mobile 0;
	@include tablet {
	  margin: $increment8-tablet 0;
	}
	@include desktop {
	  margin: $increment8--desktop 0;
	}
  }
  
  @mixin increment-9-vertical {
	margin: $increment9-mobile 0;
	@include tablet {
	  margin: $increment9-tablet 0;
	}
	@include desktop {
	  margin: $increment9--desktop 0;
	}
  }
  
  @mixin increment-10-vertical {
	margin: $increment10-mobile 0;
	@include tablet {
	  margin: $increment10-tablet 0;
	}
	@include desktop {
	  margin: $increment10--desktop 0;
	}
  }
  
  /************************************HORIZONTAL MARGIN INCREMENTS******************************************************/
  
  @mixin increment-0-horizontal {
	margin: 0 $increment0-mobile;
	@include tablet {
	  margin: 0 $increment0-tablet;
	}
	@include deskhorizontal {
	  margin: 0 $increment0-desktop;
	}
  }
  
  @mixin increment-1-horizontal {
	margin: 0 $increment1-mobile;
	@include tablet {
	  margin: 0 $increment1-tablet;
	}
	@include desktop {
	  margin: 0 $increment1-desktop;
	}
  }
  
  @mixin increment-2-horizontal {
	margin: 0 $increment2-mobile;
	@include tablet {
	  margin: 0 $increment2-tablet;
	}
	@include desktop {
	  margin: 0 $increment2-desktop;
	}
  }
  
  @mixin increment-3-horizontal {
	margin: 0 $increment3-mobile;
	@include tablet {
	  margin: 0 $increment3-tablet;
	}
	@include desktop {
	  margin: 0 $increment3-desktop;
	}
  }
  
  @mixin increment-4-horizontal {
	margin: 0 $increment4-mobile;
	@include tablet {
	  margin: 0 $increment4-tablet;
	}
	@include desktop {
	  margin: 0 $increment4-desktop;
	}
  }
  
  @mixin increment-5-horizontal {
	margin: 0 $increment5-mobile;
	@include tablet {
	  margin: 0 $increment5-tablet;
	}
	@include desktop {
	  margin: 0 $increment5-desktop;
	}
  }
  
  @mixin increment-6-horizontal {
	margin: 0 $increment6-mobile;
	@include tablet {
	  margin: 0 $increment6-tablet;
	}
	@include desktop {
	  margin: 0 $increment6-desktop;
	}
  }
  
  @mixin increment-7-horizontal {
	margin: 0 $increment7-mobile;
	@include tablet {
	  margin: 0 $increment7-tablet;
	}
	@include desktop {
	  margin: 0 $increment7-desktop;
	}
  }
  
  @mixin increment-8-horizontal {
	margin: 0 $increment8-mobile;
	@include tablet {
	  margin: 0 $increment8-tablet;
	}
	@include desktop {
	  margin: 0 $increment8--desktop;
	}
  }
  
  @mixin increment-9-horizontal {
	margin: 0 $increment9-mobile;
	@include tablet {
	  margin: 0 $increment9-tablet;
	}
	@include desktop {
	  margin: 0 $increment9--desktop;
	}
  }
  
  @mixin increment-10-horizontal {
	margin: 0 $increment10-mobile;
	@include tablet {
	  margin: 0 $increment10-tablet;
	}
	@include desktop {
	  margin: 0 $increment10--desktop;
	}
  }
    /***************************************************INCREMENT WIDTH******************************************/
  
	@mixin increment-0-width {
		width: $increment0-mobile;
		@include tablet {
		  width: $increment0-tablet;
		}
		@include deskwidth {
		  width: $increment0-desktop;
		}
	  }
	  
	  @mixin increment-1-width {
		width: $increment1-mobile;
		@include tablet {
		  width: $increment1-tablet;
		}
		@include desktop {
		  width: $increment1-desktop;
		}
	  }
	  
	  @mixin increment-2-width {
		width: $increment2-mobile;
		@include tablet {
		  width: $increment2-tablet;
		}
		@include desktop {
		  width: $increment2-desktop;
		}
	  }
	  
	  @mixin increment-3-width {
		width: $increment3-mobile;
		@include tablet {
		  width: $increment3-tablet;
		}
		@include desktop {
		  width: $increment3-desktop;
		}
	  }
	  
	  @mixin increment-4-width {
		width: $increment4-mobile;
		@include tablet {
		  width: $increment4-tablet;
		}
		@include desktop {
		  width: $increment4-desktop;
		}
	  }
	  
	  @mixin increment-5-width {
		width: $increment5-mobile;
		@include tablet {
		  width: $increment5-tablet;
		}
		@include desktop {
		  width: $increment5-desktop;
		}
	  }
	  
	  @mixin increment-6-width {
		width: $increment6-mobile;
		@include tablet {
		  width: $increment6-tablet;
		}
		@include desktop {
		  width: $increment6-desktop;
		}
	  }
	  
	  @mixin increment-7-width {
		width: $increment7-mobile;
		@include tablet {
		  width: $increment7-tablet;
		}
		@include desktop {
		  width: $increment7-desktop;
		}
	  }
	  
	  @mixin increment-8-width {
		width: $increment8-mobile;
		@include tablet {
		  width: $increment8-tablet;
		}
		@include desktop {
		  width: $increment8--desktop;
		}
	  }
	  
	  @mixin increment-9-width {
		width: $increment9-mobile;
		@include tablet {
		  width: $increment9-tablet;
		}
		@include desktop {
		  width: $increment9--desktop;
		}
	  }
	  
	  @mixin increment-10-width {
		width: $increment10-mobile;
		@include tablet {
		  width: $increment10-tablet;
		}
		@include desktop {
		  width: $increment10--desktop;
		}
	  }
  
  /***************************************************INCREMENT HEIGHT******************************************/
  
  @mixin increment-0-height {
	height: $increment0-mobile;
	@include tablet {
	  height: $increment0-tablet;
	}
	@include deskheight {
	  height: $increment0-desktop;
	}
  }
  
  @mixin increment-1-height {
	height: $increment1-mobile;
	@include tablet {
	  height: $increment1-tablet;
	}
	@include desktop {
	  height: $increment1-desktop;
	}
  }
  
  @mixin increment-2-height {
	height: $increment2-mobile;
	@include tablet {
	  height: $increment2-tablet;
	}
	@include desktop {
	  height: $increment2-desktop;
	}
  }
  
  @mixin increment-3-height {
	height: $increment3-mobile;
	@include tablet {
	  height: $increment3-tablet;
	}
	@include desktop {
	  height: $increment3-desktop;
	}
  }
  
  @mixin increment-4-height {
	height: $increment4-mobile;
	@include tablet {
	  height: $increment4-tablet;
	}
	@include desktop {
	  height: $increment4-desktop;
	}
  }
  
  @mixin increment-5-height {
	height: $increment5-mobile;
	@include tablet {
	  height: $increment5-tablet;
	}
	@include desktop {
	  height: $increment5-desktop;
	}
  }
  
  @mixin increment-6-height {
	height: $increment6-mobile;
	@include tablet {
	  height: $increment6-tablet;
	}
	@include desktop {
	  height: $increment6-desktop;
	}
  }
  
  @mixin increment-7-height {
	height: $increment7-mobile;
	@include tablet {
	  height: $increment7-tablet;
	}
	@include desktop {
	  height: $increment7-desktop;
	}
  }
  
  @mixin increment-8-height {
	height: $increment8-mobile;
	@include tablet {
	  height: $increment8-tablet;
	}
	@include desktop {
	  height: $increment8--desktop;
	}
  }
  
  @mixin increment-9-height {
	height: $increment9-mobile;
	@include tablet {
	  height: $increment9-tablet;
	}
	@include desktop {
	  height: $increment9--desktop;
	}
  }
  
  @mixin increment-10-height {
	height: $increment10-mobile;
	@include tablet {
	  height: $increment10-tablet;
	}
	@include desktop {
	  height: $increment10--desktop;
	}
  }
  
  /***************************************************OTHER COMMON INCREMENTS********************************/
  
  /*******TOP****/
  
  @mixin increment20px-top {
	margin-top: $increment20px-mobile;
	@include tablet {
	  margin-top: $increment20px-tablet;
	}
	@include desktop {
	  margin-top: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-top {
	margin-top: $increment54px-mobile;
	@include tablet {
	  margin-top: $increment54px-tablet;
	}
	@include desktop {
	  margin-top: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-top {
	margin-top: $increment68px-mobile;
	@include tablet {
	  margin-top: $increment68px-tablet;
	}
	@include desktop {
	  margin-top: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-top {
	margin-top: $increment100px-mobile;
	@include tablet {
	  margin-top: $increment100px-tablet;
	}
	@include desktop {
	  margin-top: $increment100px-desktop;
	}
  }
  
  /***************RIGHT********************/
  
  @mixin increment20px-right {
	margin-right: $increment20px-mobile;
	@include tablet {
	  margin-right: $increment20px-tablet;
	}
	@include desktop {
	  margin-right: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-right {
	margin-right: $increment54px-mobile;
	@include tablet {
	  margin-right: $increment54px-tablet;
	}
	@include desktop {
	  margin-right: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-right {
	margin-right: $increment68px-mobile;
	@include tablet {
	  margin-right: $increment68px-tablet;
	}
	@include desktop {
	  margin-right: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-right {
	margin-right: $increment100px-mobile;
	@include tablet {
	  margin-right: $increment100px-tablet;
	}
	@include desktop {
	  margin-right: $increment100px-desktop;
	}
  }
  
  /****************BOTTOM***************/
  
  @mixin increment20px-bottom {
	margin-bottom: $increment20px-mobile;
	@include tablet {
	  margin-bottom: $increment20px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-bottom {
	margin-bottom: $increment54px-mobile;
	@include tablet {
	  margin-bottom: $increment54px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-bottom {
	margin-bottom: $increment68px-mobile;
	@include tablet {
	  margin-bottom: $increment68px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-bottom {
	margin-bottom: $increment100px-mobile;
	@include tablet {
	  margin-bottom: $increment100px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment100px-desktop;
	}
  }
  
  /*****************LEFT****************/
  
  @mixin increment20px-left {
	margin-left: $increment20px-mobile;
	@include tablet {
	  margin-left: $increment20px-tablet;
	}
	@include desktop {
	  margin-left: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-left {
	margin-left: $increment54px-mobile;
	@include tablet {
	  margin-left: $increment54px-tablet;
	}
	@include desktop {
	  margin-left: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-left {
	margin-left: $increment68px-mobile;
	@include tablet {
	  margin-left: $increment68px-tablet;
	}
	@include desktop {
	  margin-left: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-left {
	margin-left: $increment100px-mobile;
	@include tablet {
	  margin-left: $increment100px-tablet;
	}
	@include desktop {
	  margin-left: $increment100px-desktop;
	}
  }
  
  /************VERTICAL***********/
  
  @mixin increment20px-vertical {
	margin: $increment20px-mobile 0;
	@include tablet {
	  margin: $increment20px-tablet 0;
	}
	@include desktop {
	  margin: $increment20px-desktop 0;
	}
  }
  
  @mixin increment54px-vertical {
	margin: $increment54px-mobile 0;
	@include tablet {
	  margin: $increment54px-tablet 0;
	}
	@include desktop {
	  margin: $increment54px-desktop 0;
	}
  }
  
  @mixin increment68px-vertical {
	margin: $increment68px-mobile 0;
	@include tablet {
	  margin: $increment68px-tablet 0;
	}
	@include desktop {
	  margin: $increment68px-desktop 0;
	}
  }
  
  @mixin increment100px-vertical {
	margin: $increment100px-mobile 0;
	@include tablet {
	  margin: $increment100px-tablet 0;
	}
	@include desktop {
	  margin: $increment100px-desktop 0;
	}
  }
  
  /**********HORIZONTAL*********/
  
  @mixin increment20px-horizontal {
	margin: 0 $increment20px-mobile;
	@include tablet {
	  margin: 0 $increment20px-tablet;
	}
	@include desktop {
	  margin: 0 $increment20px-desktop;
	}
  }
  
  @mixin increment54px-horizontal {
	margin: 0 $increment54px-mobile;
	@include tablet {
	  margin: 0 $increment54px-tablet;
	}
	@include desktop {
	  margin: 0 $increment54px-desktop;
	}
  }
  
  @mixin increment68px-horizontal {
	margin: 0 $increment68px-mobile;
	@include tablet {
	  margin: 0 $increment68px-tablet;
	}
	@include desktop {
	  margin: 0 $increment68px-desktop;
	}
  }
  
  @mixin increment100px-horizontal {
	margin: 0 $increment100px-mobile;
	@include tablet {
	  margin: 0 $increment100px-tablet;
	}
	@include desktop {
	  margin: 0 $increment100px-desktop;
	}
  }
  
 /***********WIDTH*********/
  
  @mixin increment20px-width {
	width: $increment20px-mobile;
	@include tablet {
	  width: $increment20px-tablet;
	}
	@include desktop {
	  width: $increment20px-desktop;
	}
  }

 @mixin increment54px-width {
	width: $increment54px-mobile;
	@include tablet {
	  width: $increment54px-tablet;
	}
	@include desktop {
	  width: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-width {
	width: $increment68px-mobile;
	@include tablet {
	  width: $increment68px-tablet;
	}
	@include desktop {
	  width: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-width {
	width: $increment100px-mobile;
	@include tablet {
	  width: $increment100px-tablet;
	}
	@include desktop {
	  width: $increment100px-desktop;
	}
  }

  /***********HEIGHT*********/
  @mixin increment20px-height {
	height: $increment20px-mobile;
	@include tablet {
	  height: $increment20px-tablet;
	}
	@include desktop {
	  height: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-height {
	height: $increment54px-mobile;
	@include tablet {
	  height: $increment54px-tablet;
	}
	@include desktop {
	  height: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-height {
	height: $increment68px-mobile;
	@include tablet {
	  height: $increment68px-tablet;
	}
	@include desktop {
	  height: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-height {
	height: $increment100px-mobile;
	@include tablet {
	  height: $increment100px-tablet;
	}
	@include desktop {
	  height: $increment100px-desktop;
	}
  }
  
  /**********************************************MOCKUP COLUMNS PADDING****************************************************************/
  
  @mixin mockup-column-padding-outside-right {
	padding-right: $mockup-column-outside-mobile;
	@include tablet {
	  padding-right: $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-right {
	padding-right: $mockup-column-small-mobile;
	@include tablet {
	  padding-right: $mockup-column-small-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-right {
	padding-right: $mockup-column-large-mobile;
	@include tablet {
	  padding-right: $mockup-column-large-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-large-desktop;
	}
  }
  
  /***LEFT*******/
  
  @mixin mockup-column-padding-outside-left {
	padding-left: $mockup-column-outside-mobile;
	@include tablet {
	  padding-left: $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-left {
	padding-left: $mockup-column-small-mobile;
	@include tablet {
	  padding-left: $mockup-column-small-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-left {
	padding-left: $mockup-column-large-mobile;
	@include tablet {
	  padding-left: $mockup-column-large-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-large-desktop;
	}
  }
  
  /***HORIZONTAL*******/
  
  @mixin mockup-column-padding-outside-horizontal {
	padding: 0 $mockup-column-outside-mobile;
	@include tablet {
	  padding: 0 $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-horizontal {
	padding: 0 $mockup-column-small-mobile;
	@include tablet {
	  padding: 0 $mockup-column-small-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-horizontal {
	padding: 0 $mockup-column-large-mobile;
	@include tablet {
	  padding: 0 $mockup-column-large-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-large-desktop;
	}
  }
  /*************************************TEXT STYLES************************************/
@use "./colors" as *;
@use "./breakpoints" as *;
@use "./variables" as *;
/*****************************************H1-Main-heading***************************/

@mixin h1-main-header {
    font-family: AvenirNextLTProBold;
    font-size: 1.5rem; //24px 
    line-height: 2.0rem; //32px
    color: $white;
    @include tablet {
        font-size: 2.3rem; //36px 2.25rem 
        line-height: 2.8rem; //44px
    }
    @include desktop {
        font-size: 2.3rem; //36px 2.25rem 
        line-height: 2.8rem; //44px
    }
}

@mixin h1-main-header-regular {
    @include h1-main-header;
    font-family: AvenirNextLTProRegular;
}

@mixin h1-main-header-black {
    @include h1-main-header;
    color: $black;
}

@mixin h1-desktop-size-that-should-be-available-on-mobile {
    color: $white;
    font-family: AvenirNextLTProBold;
    font-size: 8.0vw;
    line-height: 10.8vw;
}


/*********************H2 - Header subheader for mobile*********************/

@mixin h2-header {
    font-size: 0.9rem; //14px 
    line-height: 1.4rem; //22px 
    font-family: AvenirNextLTProDemi;
    color: $black;
    @include tablet {
        font-size: 1.5rem; //24px 1.5rem 
        line-height: 2.0; //32px 2rem
        font-family: AvenirNextLTProBold;
    }
    @include desktop {
        font-size: 1.5rem; //24px 1.5rem 
        line-height: 2.0rem; //32px 2rem 
    }
}

@mixin h2-header-white {
    @include h2-header;
    color: $white;
}

@mixin h2-header-responsive-white {
    @include h2-header-responsive;
    color: $white;
}

@mixin h2-desktop-size-that-should-be-available-on-mobile {
    color: $white;
    font-size: 1.5rem; //24px 1.5rem 
    line-height: 2.0; //32px 2rem
    font-family: AvenirNextLTProBold;
}


/*******************************H3 - Subheader for Tablet and Desktop************/

@mixin h3-subheader {
    font-size: 1.0rem; //16px 
    line-height: 1.5rem; // 24px
    font-family: AvenirNextLTProDemi;
    color: $black;
    @include tablet {
        font-size: 1.0rem; //16px 
        line-height: 1.5rem; // 24px
    }
    @include desktop {
        font-size: 1.0rem; //16px 
        line-height: 1.5rem; // 24px
    }
}

@mixin h3-subheader-white {
    @include h3-subheader;
    color: $white;
}

@mixin h3-subheader-bold {
    @include h3-subheader;
    font-family: AvenirNextLTProBold;
}

@mixin h3-subheader-white-bold {
    @include h3-subheader;
    font-family: AvenirNextLTProBold;
    color: $white;
}


/********************H4 Paragraph & Comments******************************/

@mixin h4-paragraphs-and-comments {
    font-family: AvenirNextLTProRegular;
    color: $black;
    font-size: 0.8rem; // 13px
    line-height: 1.1rem; //18px
    @include tablet {
        font-size: 0.9rem; // 14px
        line-height: 1.4rem; //22px
    }
    @include desktop {
        font-size: 0.9rem; // 14px
        line-height: 1.4rem; // 22px 
    }
}

@mixin h4-paragraphs-and-comments-bold {
    @include h4-paragraphs-and-comments;
    font-family: AvenirNextLTProBold;
}

@mixin h4-paragraphs-and-comments-white {
    @include h4-paragraphs-and-comments;
    color: $white;
}

@mixin h4-paragraphs-and-comments-white-bold {
    @include h4-paragraphs-and-comments-bold;
    color: $white;
}

@mixin h4-paragraphs-and-comments-white-demi {
    @include h4-paragraphs-and-comments-white;
    font-family: AvenirNextLTProDemi;
    color: $white;
}

$time: 330ms;
$btns: ( 
alpha: ( gstart: #21D4FD,
gend: #B721FF,
tstart: #B721FF,
tend: #21D4FD),
beta: ( gstart: #08AEEA,
gend: #2AF598,
tstart: #2AF598,
tend: #08AEEA),
gamma: ( gstart: #FEE140,
gend: #FA709A,
tstart: #FA709A,
tend: #FEE140),
delta: ( gstart: #3EECAC,
gend: #EE74E1,
tstart: #EE74E1,
tend: #3EECAC));
.btn {
	outline: none;;
	margin-right: 30px;
	margin-top: 30px;
	font-family: 'Source Sans Pro', sans-serif;
	font-weight: 900;
	padding: 1.25rem 2rem;
	font-size: 1rem;
	border-radius: 3.5rem / 100%;
	position: relative;
	min-width: 15rem;
	max-width: 90vw;
	overflow: hidden;
	border: 0;
	cursor: pointer;
	text-transform: uppercase;
	letter-spacing: 0.05em;
	transition: all $time;
	outline: none;;
	&+& {
		outline: none;;
		margin-top: 1rem;
	}
	span {
		outline: none;;
		position: relative;
		z-index: 1;
	}
	&:before {
		outline: none;;
		content: "";
		background-color: #21D4FD;
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		transform: scale(4) translateX(-100%);
		transition: all $time * 1.5 ease-out;
		outline: none;;
	}
	&:hover,
	&:focus,
	&:active {
		&:before {
			outline: none;;
			transform: scale(4) translate(37%);
		}
	}
}

@each $bname,
$bcolors in $btns {
	.btn--#{$bname} {
		outline: none;;
		background-color: map-get($bcolors, gstart);
		color: map-get($bcolors, tstart);
		&:before {
			background-color: map-get($bcolors, gend);
			background-image: linear-gradient(to right, map-get($bcolors, gend) 30%, map-get($bcolors, gstart) 100%);
			position: absolute;
		}
		&:hover,
		&:focus,
		&:active {
			outline: none;;
			color: map-get($bcolors, tend);
		}
	}
}

.btn-form {
	outline: none;;
	outline: none;;
	margin-right: 30px;
	margin-top: 30px;
	font-family: 'Source Sans Pro', sans-serif;
	font-weight: 900;
	padding: 1.25rem 2rem;
	font-size: 1rem;
	border-radius: 3.5rem / 100%;
	position: relative;
	min-width: 15rem;
	max-width: 90vw;
	overflow: hidden;
	border: 0;
	cursor: pointer;
	text-transform: uppercase;
	letter-spacing: 0.05em;
	transition: all $time;
	 ;
	&+& {
		margin-top: 1rem;
		 ;
	}
	span {
		position: relative;
		z-index: 1;
		 ;
	}
	&:before {
		content: "";
		background-color: #21D4FD;
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		transform: scale(4) translateX(-100%);
		transition: all $time * 1.5 ease-out;
		 ;
	}
	&:hover,
	&:focus,
	&:active {
		&:before {
			 
			//transform: scale(4) translate(37%);
		  //  color: map-get($bcolors, tstart);
		  background-color:#B721FF;;
		}
	}
}

@each $bname,
$bcolors in $btns {
	.btn-form--#{$bname} {
		background-color: map-get($bcolors, gstart);
		color: map-get($bcolors, tstart);
		&:before {
			background-color: map-get($bcolors, gend);
			background-image: linear-gradient(to right, map-get($bcolors, gend) 30%, map-get($bcolors, gstart) 100%);
			position: absolute;
		}
		&:hover,
		&:focus,
		&:active {
		//	color: map-get($bcolors, tstart);
		color: #B721FF;
		}
	}
}
/********************************h4-lists-and-contacts-information*************/

@mixin h4-list-contact-information {
    color: $black;
    font-family: AvenirNextLTProRegular;
    font-size: 0.8rem; //13px 
    line-height: 1.1rem; //18px 
    @include tablet {
        font-size: 0.9rem; //14px 
        line-height: 1.3rem; //20px
    }
    @include desktop {
        font-size: 0.9rem; //14px 
        line-height: 1.3rem; //20px 
    }
}

@mixin h4-list-contact-information-bold {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProBold;
}

@mixin h4-list-contact-information-demi {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProDemi;
}

@mixin h4-list-contact-information-white {
    @include h4-list-contact-information;
    color: $white;
}

@mixin h4-list-contact-information-white-demi {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProDemi;
    color: $white;
}

@mixin h4-list-contact-information-white-bold {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProBold;
    color: $white;
}


//********************************P-Paragraph****************************/

@mixin p-paragraph {
    font-family: AvenirNextLTProRegular;
    color: $black;
    font-size: 0.8rem; //13px
    line-height: 1.1rem; //18px
    @include tablet {
        font-size: 0.9rem; //13px 
        line-height: 1.3rem; //18px 
        display: inline-block;
    }
    @include desktop {
        font-size: 0.9rem; //13px 
        line-height: 1.3rem; //18px
        display: inline-block;
    }
}

@mixin p-paragraph-bold {
    @include p-paragraph;
    font-family: AvenirNextLTProBold;
}

@mixin p-paragraph-white {
    @include p-paragraph;
    color: white;
}

@mixin p-paragraph-white-bold {
    @include p-paragraph-bold;
    color: white;
}

@mixin h1-page-header {
	font-family: TitilliumWebSemiBold600;
	color: $black;
	font-size: 28px; //28px
	line-height: 36px; //36px
	@include tablet {
        font-size: 32px; //32px
        line-height: 40px; //40px
	}
	@include desktop {
        font-size: 32px; //32px
        line-height: 40px; //40px
	}
}

@mixin h2-subheader {
	font-family: TitilliumWebSemiBold600;
	color: $black;
	font-size: 20px; //20px
	line-height: 28px; //28px
	@include tablet {
        font-size: 24px; //24px
        line-height: 32px; //28px
	}
	@include desktop {
        font-size: 24px; //24px
        line-height: 32px; //28px
	}
}
@mixin h3-labels-links-buttons {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 13px; //13px
	line-height: 20px; //20px
	@include tablet {
        font-size: 14px; //14px
        line-height: 22px; //22px
	}
	@include desktop {
        font-size: 14px; //14px
        line-height: 22px; //22px
	}
}
@mixin h4-labels-table-header {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 11px; //11px
	line-height: 16px; //16px
	@include tablet {
        font-size: 12px; //12px
        line-height: 18px; //18px
	}
	@include desktop {
        font-size: 12px; //12px
        line-height: 18px; //18px
	}
}
@mixin p1-body-large {
	font-family: TitilliumWebRegular400;
	color: $black;
    font-size: 15px; //15px
    line-height: 26px; //26px
	@include tablet {
        font-size: 16px; //12px
        line-height: 28px; //18px
	}
	@include desktop {
        font-size: 16px; //12px
        line-height: 28px; //18px
	}
}
@mixin p2-body-medium {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 16px; //13px
	line-height: 28px; //20px
	@include tablet {
        font-size: 13px; //13px
        line-height: 20px; //20px
	}
	@include desktop {
        font-size: 13px; //13px
        line-height: 20px; //20px
	}
}
@mixin p2-body-small {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 11px; //13px
	line-height: 16px; //20px
	@include tablet {
        font-size: 12px; //13px
        line-height: 18px; //20px
	}
	@include desktop {
        font-size: 12px; //13px
        line-height: 18px; //20px
	}
}

/*******************************************H5-Labels***********************/

@mixin h5-labels {
    font-size: 0.8rem; //12px
    line-height: 1.3rem; //20px 
    font-family: AvenirNextLTProRegular;
    color: $silver; //labels
    @include tablet {
        font-size: 0.8rem; //12px
        line-height: 1.3rem; // 20px
        font-family: AvenirNextLTProDemi;
    }
    @include desktop {
        font-size: 0.8rem; //12px
        line-height: 1.8rem; // 20px
    }
}

@mixin h5-labels-demi {
    @include h5-labels;
    font-family: AvenirNextLTProDemi;
}

@mixin h5-labels-bold {
    @include h5-labels;
    font-family: AvenirNextLTProBold;
}


//Breakpoints
@mixin tablet {
    @media (min-width: $breakpoint-tablet) {
        @content;
    }
}

@mixin desktop {
    @media (min-width: $breakpoint-desktop) {
        @content;
    }
}

//Text Styles- Desktop

@mixin d-header {
    font-family: $font-family;
    font-size: 1.875rem;
    line-height: 2.25rem;
    font-weight: 700;
	color: $text-black;
}

@mixin d-subheader {
	font-family: $font-family;
    font-size: 1.875rem;
    line-height: 2.25rem;
    font-weight: 500;
	color: $text-black;
}

@mixin d-subheader-grey {
	font-family: $font-family;
    font-size: 1.75rem;
    line-height: 2rem;
	color: $text-grey;
    font-weight: 500;
}

@mixin d-paragraph-bold {
	font-family: $font-family;
    font-size: 1.125rem;
    line-height: 1.25rem;
	color: $text-black;
    font-weight: 700;
}

@mixin d-paragraph-large {
	font-family: $font-family;
    font-size: 1.375rem;
    line-height: 1.4rem;
	color: $text-grey;
    font-weight: 400;
}

@mixin d-paragraph-medium {
	font-family: $font-family;
    font-size: 1.25rem;
    line-height: 1.4rem;
	color: $text-grey;
    font-weight: 400;
}

@mixin d-paragraph-small {
	font-family: $font-family;
    font-size: 1.4rem;
    line-height: 1rem;
	color: $text-grey;
    font-weight: 400;
}

//Button Styling
@mixin button-blue {
    background-color: $secondary-color-indigo;
    border-radius: 5px;
    color: white;
    padding: 0.875rem 1.813rem;
    appearance: none;
    border: none;
}

//Component Styling

@mixin component-styling {
    box-shadow: 0px 10px 25px rgba(19, 24, 44, 0.1);
    border-radius: 10px;
    padding: 3.75rem;
}
/***************************************STANDARD PADDING**************************************************/
@mixin standard-padding-right {
	padding-right: $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding-right: $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding-right: $mockup-column-outside-desktop; //80px right 80px left
	}
  }

  @mixin standard-padding-left {
	padding-left: $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding-left: $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding-left: $mockup-column-outside-desktop; //80px right 80px left
	}
  }
  
  @mixin standard-padding-horizontal {
	padding: 0 $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding: 0 $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding: 0 $mockup-column-outside-desktop; //80px right 80px left
	}
  }
  
  
  /**********************************WIDTH*100%*PADDING**********************************************/
  
  @mixin width-100-percent-minus-standard-padding-horizontal {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent} * 2));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent} * 2));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent} * 2));
	}
  }
  
  @mixin width-100-percent-minus-standard-padding-left {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent}));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent}));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent}));
	}
  }
  
  @mixin width-100-percent-minus-standard-padding-right {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent}));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent}));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent}));
	}
  }
  

  /*******************WIDTH MINUS MOCKUP COLUMN************/
  @mixin width-100-percent-minus-mockup-column-outside {
	width: calc(100% - #{$mockup-column-outside-mobile});
	@include tablet {
	  width: calc(100% - #{$mockup-column-outside-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-outside-desktop});
	}
  }
  @mixin width-100-percent-minus-mockup-column-outside-horizontal {
	width: calc(100% - (#{$mockup-column-outside-mobile} * 2));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet} *2));
	}
	@include desktop {
	  width: calc(100% - (#{mockup-column-outside-desktop} *2));
	}
  }
  @mixin width-100-percent-minus-mockup-column-small {
	width: calc(100% - #{mockup-column-small-mobile});
	@include tablet {
	  width: calc(100% -#{mockup-column-small-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-small-desktop});
	}
  }
  
  @mixin width-100-percent-minus-mockup-column-large {
	width: calc(100% - #{mockup-column-large-mobile});
	@include tablet {
	  width: calc(100% -#{mockup-column-large-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-large-desktop});
	}
  }
  



@mixin fancybutton {
  font-family: 'Source Sans Pro', sans-serif;
  font-weight: 900;
  padding: 1.25rem 2rem;
  font-size: 1rem;
  border-radius: 3.5rem / 100%;
  position: relative;
  min-width: 15rem;
  max-width: 90vw;
  overflow: hidden;
  border: 0;
  cursor: pointer;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  transition: all $time;
  
  & + & {
    margin-top: 1rem;
  }
  
  span {
    position: relative;
    z-index: 1;
  }
  
  &:before {
    content: "";
    background-color: #21D4FD;
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    transform: scale(4) translateX(-100%);
    transition: all $time * 1.5 ease-out;
  }
  
  &:hover,
  &:focus,
  &:active {    
    &:before {
      transform: scale(4) translate(37%);
    } 
  }
}


  /**********************************************DIVIDERS******************************************************/
  
  @mixin divider {
	background: $silver;
	width: 100%;
	height: 1px;
  }
  
  @mixin divider-form {
	background: $silver;
	width: 100%;
	height: 1px;
	margin-bottom: $increment2-px;
  }
  
  /**********************************************LOGOS******************************************************/
  
  @mixin logo-brainflix {
	img {
	  width: 120px; //120px
	  height: 24px; //24px
	  margin-left: 1rem;
	  @include tablet {
		margin-left: 2rem;
		width: 120px; //120px
		height: 24px; //24px
	  }
	  @include desktop {
		margin-left: 5rem;
	  }
	}
  }
  
  /**********************************************BUTTONS******************************************************/
  
  @mixin button {
	background-color: $brainflix-blue;
	@include h4-paragraphs-and-comments-white-demi;
	&:hover {
	  background: $brainflix-dark-blue-hover;
	}
	&:focus {
	  outline: none;
	}
	width: $increment28-px; //226px
	height: $increment6-px; 
	border-radius: $increment0-px; 
	border-color: $brainflix-blue;
	@include tablet {
	  padding: 0;
	  width: $increment20-px; //160px
	  height: $increment6-px; 
	}
  }
  
  /**********************************************MOCKUP COLUMNS****************************************************************/
  
  /*******************RIGHT*****************/
  
  @mixin mockup-column-outside-right {
	margin-right: $mockup-column-outside-mobile;
	@include tablet {
	  margin-right: $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-right {
	margin-right: $mockup-column-small-mobile;
	@include tablet {
	  margin-right: $mockup-column-small-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-right {
	margin-right: $mockup-column-large-mobile;
	@include tablet {
	  margin-right: $mockup-column-large-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-large-desktop;
	}
  }
  
  /*******************LEFT***************/
  
  @mixin mockup-column-outside-left {
	margin-left: $mockup-column-outside-mobile;
	@include tablet {
	  margin-left: $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-left {
	margin-left: $mockup-column-small-mobile;
	@include tablet {
	  margin-left: $mockup-column-small-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-left {
	margin-left: $mockup-column-large-mobile;
	@include tablet {
	  margin-left: $mockup-column-large-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-large-desktop;
	}
  }
  
  /*********************HORIZONTAL**************/
  
  @mixin mockup-column-outside-horizontal {
	margin: 0 $mockup-column-outside-mobile;
	@include tablet {
	  margin: 0 $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-horizontal {
	margin: 0 $mockup-column-small-mobile;
	@include tablet {
	  margin: 0 $mockup-column-small-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-horizontal {
	margin: 0 $mockup-column-large-mobile;
	@include tablet {
	  margin: 0 $mockup-column-large-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-large-desktop;
	}
  }
  
  /*********************************************WIDTH 100% MINUS INCREMENTS*********************************/
  
  @mixin width-100-percent-minus-increment0 {
	width: calc(100% - #{increment0-mobile});
	@include tablet {
	  width: calc(100% -#{increment0-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment0-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment1 {
	width: calc(100% - #{increment1-mobile});
	@include tablet {
	  width: calc(100% - #{increment1-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment1-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment2 {
	width: calc(100% - #{increment2-mobile});
	@include tablet {
	  width: calc(100% -#{increment2-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment2-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment3 {
	width: calc(100% - #{increment3-mobile});
	@include tablet {
	  width: calc(100% -#{increment3-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment3-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment4 {
	width: calc(100% - #{increment4-mobile});
	@include tablet {
	  width: calc(100% -#{increment4-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment4-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment5 {
	width: calc(100% - #{increment5-mobile});
	@include tablet {
	  width: calc(100% -#{increment5-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment5-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment6 {
	width: calc(100% - #{increment6-mobile});
	@include tablet {
	  width: calc(100% -#{increment6-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment6-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment7 {
	width: calc(100% - #{increment7-mobile});
	@include tablet {
	  width: calc(100% -#{increment7-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment7-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment8 {
	width: calc(100% - #{increment8-mobile});
	@include tablet {
	  width: calc(100% -#{increment8-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment8-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment9 {
	width: calc(100% - #{increment9-mobile});
	@include tablet {
	  width: calc(100% -#{increment9-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment9-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment10 {
	width: calc(100% - #{increment10-mobile});
	@include tablet {
	  width: calc(100% -#{increment10-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment10-desktop});
	}
  }
  

  
  /************************************%%%%%%%%%%%%%%%%%%%%%******************************************************
  /******%%%%%%%%%%%%%%%%%%%%%%%****CONTENTS LIST:*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****TOP MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *****%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%******RIGHT MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***BOTTOM MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***LEFT MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ***********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*VERTICAL MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ***********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*HORIZTONAL**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***WIDTH **********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%****HEIGHT**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****OTHER COMMON INCREMENTS*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
   ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****MOCKUP COLUMN PADDING*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********/
  
  /*******************************************************INCREMENT MARGIN TOP******************************************/
  
  @mixin increment-0-top {
	margin-top: $increment0-mobile;
	@include tablet {
	  margin-top: $increment0-tablet;
	}
	@include desktop {
	  margin-top: $increment0-desktop;
	}
  }
  
  @mixin increment-1-top {
	margin-top: $increment1-mobile;
	@include tablet {
	  margin-top: $increment1-tablet;
	}
	@include desktop {
	  margin-top: $increment1-desktop;
	}
  }
  
  @mixin increment-2-top {
	margin-top: $increment2-mobile;
	@include tablet {
	  margin-top: $increment2-tablet;
	}
	@include desktop {
	  margin-top: $increment2-desktop;
	}
  }
  
  @mixin increment-3-top {
	margin-top: $increment3-mobile;
	@include tablet {
	  margin-top: $increment3-tablet;
	}
	@include desktop {
	  margin-top: $increment3-desktop;
	}
  }
  
  @mixin increment-4-top {
	margin-top: $increment4-mobile;
	@include tablet {
	  margin-top: $increment4-tablet;
	}
	@include desktop {
	  margin-top: $increment4-desktop;
	}
  }
  
  @mixin increment-5-top {
	margin-top: $increment5-mobile;
	@include tablet {
	  margin-top: $increment5-tablet;
	}
	@include desktop {
	  margin-top: $increment5-desktop;
	}
  }
  
  @mixin increment-6-top {
	margin-top: $increment6-mobile;
	@include tablet {
	  margin-top: $increment6-tablet;
	}
	@include desktop {
	  margin-top: $increment6-desktop;
	}
  }
  
  @mixin increment-7-top {
	margin-top: $increment7-mobile;
	@include tablet {
	  margin-top: $increment7-tablet;
	}
	@include desktop {
	  margin-top: $increment7-desktop;
	}
  }
  
  @mixin increment-8-top {
	margin-top: $increment8-mobile;
	@include tablet {
	  margin-top: $increment8-tablet;
	}
	@include desktop {
	  margin-top: $increment8--desktop;
	}
  }
  
  @mixin increment-9-top {
	margin-top: $increment9-mobile;
	@include tablet {
	  margin-top: $increment9-tablet;
	}
	@include desktop {
	  margin-top: $increment9--desktop;
	}
  }
  
  @mixin increment-10-top {
	margin-top: $increment10-mobile;
	@include tablet {
	  margin-top: $increment10-tablet;
	}
	@include desktop {
	  margin-top: $increment10--desktop;
	}
  }
  
  /***********************************INCREMENT RIGHT*************************************************/
  
  @mixin increment-0-right {
	margin-right: $increment0-mobile;
	@include tablet {
	  margin-right: $increment0-tablet;
	}
	@include desktop {
	  margin-right: $increment0-desktop;
	}
  }
  
  @mixin increment-1-right {
	margin-right: $increment1-mobile;
	@include tablet {
	  margin-right: $increment1-tablet;
	}
	@include desktop {
	  margin-right: $increment1-desktop;
	}
  }
  
  @mixin increment-2-right {
	margin-right: $increment2-mobile;
	@include tablet {
	  margin-right: $increment2-tablet;
	}
	@include desktop {
	  margin-right: $increment2-desktop;
	}
  }
  
  @mixin increment-3-right {
	margin-right: $increment3-mobile;
	@include tablet {
	  margin-right: $increment3-tablet;
	}
	@include desktop {
	  margin-right: $increment3-desktop;
	}
  }
  
  @mixin increment-4-right {
	margin-right: $increment4-mobile;
	@include tablet {
	  margin-right: $increment4-tablet;
	}
	@include desktop {
	  margin-right: $increment4-desktop;
	}
  }
  
  @mixin increment-5-right {
	margin-right: $increment5-mobile;
	@include tablet {
	  margin-right: $increment5-tablet;
	}
	@include desktop {
	  margin-right: $increment5-desktop;
	}
  }
  
  @mixin increment-6-right {
	margin-right: $increment6-mobile;
	@include tablet {
	  margin-right: $increment6-tablet;
	}
	@include desktop {
	  margin-right: $increment6-desktop;
	}
  }
  
  @mixin increment-7-right {
	margin-right: $increment7-mobile;
	@include tablet {
	  margin-right: $increment7-tablet;
	}
	@include desktop {
	  margin-right: $increment7-desktop;
	}
  }
  
  @mixin increment-8-right {
	margin-right: $increment8-mobile;
	@include tablet {
	  margin-right: $increment8-tablet;
	}
	@include desktop {
	  margin-right: $increment8--desktop;
	}
  }
  
  @mixin increment-9-right {
	margin-right: $increment9-mobile;
	@include tablet {
	  margin-right: $increment9-tablet;
	}
	@include desktop {
	  margin-right: $increment9--desktop;
	}
  }
  
  @mixin increment-10-right {
	margin-right: $increment10-mobile;
	@include tablet {
	  margin-right: $increment10-tablet;
	}
	@include desktop {
	  margin-right: $increment10--desktop;
	}
  }
  
  /*******************************************************INCREMENT MARGIN BOTTOM******************************************/
  
  @mixin increment-0-bottom {
	margin-bottom: $increment0-mobile;
	@include tablet {
	  margin-bottom: $increment0-tablet;
	}
	@include desktop {
	  margin-bottom: $increment0-desktop;
	}
  }
  
  @mixin increment-1-bottom {
	margin-bottom: $increment1-mobile;
	@include tablet {
	  margin-bottom: $increment1-tablet;
	}
	@include desktop {
	  margin-bottom: $increment1-desktop;
	}
  }
  
  @mixin increment-2-bottom {
	margin-bottom: $increment2-mobile;
	@include tablet {
	  margin-bottom: $increment2-tablet;
	}
	@include desktop {
	  margin-bottom: $increment2-desktop;
	}
  }
  
  @mixin increment-3-bottom {
	margin-bottom: $increment3-mobile;
	@include tablet {
	  margin-bottom: $increment3-tablet;
	}
	@include desktop {
	  margin-bottom: $increment3-desktop;
	}
  }
  
  @mixin increment-4-bottom {
	margin-bottom: $increment4-mobile;
	@include tablet {
	  margin-bottom: $increment4-tablet;
	}
	@include desktop {
	  margin-bottom: $increment4-desktop;
	}
  }
  
  @mixin increment-5-bottom {
	margin-bottom: $increment5-mobile;
	@include tablet {
	  margin-bottom: $increment5-tablet;
	}
	@include desktop {
	  margin-bottom: $increment5-desktop;
	}
  }
  
  @mixin increment-6-bottom {
	margin-bottom: $increment6-mobile;
	@include tablet {
	  margin-bottom: $increment6-tablet;
	}
	@include desktop {
	  margin-bottom: $increment6-desktop;
	}
  }
  
  @mixin increment-7-bottom {
	margin-bottom: $increment7-mobile;
	@include tablet {
	  margin-bottom: $increment7-tablet;
	}
	@include desktop {
	  margin-bottom: $increment7-desktop;
	}
  }
  
  @mixin increment-8-bottom {
	margin-bottom: $increment8-mobile;
	@include tablet {
	  margin-bottom: $increment8-tablet;
	}
	@include desktop {
	  margin-bottom: $increment8--desktop;
	}
  }
  
  @mixin increment-9-bottom {
	margin-bottom: $increment9-mobile;
	@include tablet {
	  margin-bottom: $increment9-tablet;
	}
	@include desktop {
	  margin-bottom: $increment9--desktop;
	}
  }
  
  @mixin increment-10-bottom {
	margin-bottom: $increment10-mobile;
	@include tablet {
	  margin-bottom: $increment10-tablet;
	}
	@include desktop {
	  margin-bottom: $increment10--desktop;
	}
  }
  
  /*******************************************************INCREMENT LEFT******************************************/
  
  @mixin increment-0-left {
	margin-left: $increment0-mobile;
	@include tablet {
	  margin-left: $increment0-tablet;
	}
	@include desktop {
	  margin-left: $increment0-desktop;
	}
  }
  
  @mixin increment-1-left {
	margin-left: $increment1-mobile;
	@include tablet {
	  margin-left: $increment1-tablet;
	}
	@include desktop {
	  margin-left: $increment1-desktop;
	}
  }
  
  @mixin increment-2-left {
	margin-left: $increment2-mobile;
	@include tablet {
	  margin-left: $increment2-tablet;
	}
	@include desktop {
	  margin-left: $increment2-desktop;
	}
  }
  
  @mixin increment-3-left {
	margin-left: $increment3-mobile;
	@include tablet {
	  margin-left: $increment3-tablet;
	}
	@include desktop {
	  margin-left: $increment3-desktop;
	}
  }
  
  @mixin increment-4-left {
	margin-left: $increment4-mobile;
	@include tablet {
	  margin-left: $increment4-tablet;
	}
	@include desktop {
	  margin-left: $increment4-desktop;
	}
  }
  
  @mixin increment-5-left {
	margin-left: $increment5-mobile;
	@include tablet {
	  margin-left: $increment5-tablet;
	}
	@include desktop {
	  margin-left: $increment5-desktop;
	}
  }
  
  @mixin increment-6-left {
	margin-left: $increment6-mobile;
	@include tablet {
	  margin-left: $increment6-tablet;
	}
	@include desktop {
	  margin-left: $increment6-desktop;
	}
  }
  
  @mixin increment-7-left {
	margin-left: $increment7-mobile;
	@include tablet {
	  margin-left: $increment7-tablet;
	}
	@include desktop {
	  margin-left: $increment7-desktop;
	}
  }
  
  @mixin increment-8-left {
	margin-left: $increment8-mobile;
	@include tablet {
	  margin-left: $increment8-tablet;
	}
	@include desktop {
	  margin-left: $increment8--desktop;
	}
  }
  
  @mixin increment-9-left {
	margin-left: $increment9-mobile;
	@include tablet {
	  margin-left: $increment9-tablet;
	}
	@include desktop {
	  margin-left: $increment9--desktop;
	}
  }
  
  @mixin increment-10-left {
	margin-left: $increment10-mobile;
	@include tablet {
	  margin-left: $increment10-tablet;
	}
	@include desktop {
	  margin-left: $increment10--desktop;
	}
  }
  
  /************************************VERTICAL MARGIN INCREMENTS******************************************************/
  
  @mixin increment-0-vertical {
	margin: $increment0-mobile 0;
	@include tablet {
	  margin: $increment0-tablet 0;
	}
	@include deskvertical {
	  margin: $increment0-desktop 0;
	}
  }
  
  @mixin increment-1-vertical {
	margin: $increment1-mobile 0;
	@include tablet {
	  margin: $increment1-tablet 0;
	}
	@include desktop {
	  margin: $increment1-desktop 0;
	}
  }
  
  @mixin increment-2-vertical {
	margin: $increment2-mobile 0;
	@include tablet {
	  margin: $increment2-tablet 0;
	}
	@include desktop {
	  margin: $increment2-desktop 0;
	}
  }
  
  @mixin increment-3-vertical {
	margin: $increment3-mobile 0;
	@include tablet {
	  margin: $increment3-tablet 0;
	}
	@include desktop {
	  margin: $increment3-desktop 0;
	}
  }
  
  @mixin increment-4-vertical {
	margin: $increment4-mobile 0;
	@include tablet {
	  margin: $increment4-tablet 0;
	}
	@include desktop {
	  margin: $increment4-desktop 0;
	}
  }
  
  @mixin increment-5-vertical {
	margin: $increment5-mobile 0;
	@include tablet {
	  margin: $increment5-tablet 0;
	}
	@include desktop {
	  margin: $increment5-desktop 0;
	}
  }
  
  @mixin increment-6-vertical {
	margin: $increment6-mobile 0;
	@include tablet {
	  margin: $increment6-tablet 0;
	}
	@include desktop {
	  margin: $increment6-desktop 0;
	}
  }
  
  @mixin increment-7-vertical {
	margin: $increment7-mobile 0;
	@include tablet {
	  margin: $increment7-tablet 0;
	}
	@include desktop {
	  margin: $increment7-desktop 0;
	}
  }
  
  @mixin increment-8-vertical {
	margin: $increment8-mobile 0;
	@include tablet {
	  margin: $increment8-tablet 0;
	}
	@include desktop {
	  margin: $increment8--desktop 0;
	}
  }
  
  @mixin increment-9-vertical {
	margin: $increment9-mobile 0;
	@include tablet {
	  margin: $increment9-tablet 0;
	}
	@include desktop {
	  margin: $increment9--desktop 0;
	}
  }
  
  @mixin increment-10-vertical {
	margin: $increment10-mobile 0;
	@include tablet {
	  margin: $increment10-tablet 0;
	}
	@include desktop {
	  margin: $increment10--desktop 0;
	}
  }
  
  /************************************HORIZONTAL MARGIN INCREMENTS******************************************************/
  
  @mixin increment-0-horizontal {
	margin: 0 $increment0-mobile;
	@include tablet {
	  margin: 0 $increment0-tablet;
	}
	@include deskhorizontal {
	  margin: 0 $increment0-desktop;
	}
  }
  
  @mixin increment-1-horizontal {
	margin: 0 $increment1-mobile;
	@include tablet {
	  margin: 0 $increment1-tablet;
	}
	@include desktop {
	  margin: 0 $increment1-desktop;
	}
  }
  
  @mixin increment-2-horizontal {
	margin: 0 $increment2-mobile;
	@include tablet {
	  margin: 0 $increment2-tablet;
	}
	@include desktop {
	  margin: 0 $increment2-desktop;
	}
  }
  
  @mixin increment-3-horizontal {
	margin: 0 $increment3-mobile;
	@include tablet {
	  margin: 0 $increment3-tablet;
	}
	@include desktop {
	  margin: 0 $increment3-desktop;
	}
  }
  
  @mixin increment-4-horizontal {
	margin: 0 $increment4-mobile;
	@include tablet {
	  margin: 0 $increment4-tablet;
	}
	@include desktop {
	  margin: 0 $increment4-desktop;
	}
  }
  
  @mixin increment-5-horizontal {
	margin: 0 $increment5-mobile;
	@include tablet {
	  margin: 0 $increment5-tablet;
	}
	@include desktop {
	  margin: 0 $increment5-desktop;
	}
  }
  
  @mixin increment-6-horizontal {
	margin: 0 $increment6-mobile;
	@include tablet {
	  margin: 0 $increment6-tablet;
	}
	@include desktop {
	  margin: 0 $increment6-desktop;
	}
  }
  
  @mixin increment-7-horizontal {
	margin: 0 $increment7-mobile;
	@include tablet {
	  margin: 0 $increment7-tablet;
	}
	@include desktop {
	  margin: 0 $increment7-desktop;
	}
  }
  
  @mixin increment-8-horizontal {
	margin: 0 $increment8-mobile;
	@include tablet {
	  margin: 0 $increment8-tablet;
	}
	@include desktop {
	  margin: 0 $increment8--desktop;
	}
  }
  
  @mixin increment-9-horizontal {
	margin: 0 $increment9-mobile;
	@include tablet {
	  margin: 0 $increment9-tablet;
	}
	@include desktop {
	  margin: 0 $increment9--desktop;
	}
  }
  
  @mixin increment-10-horizontal {
	margin: 0 $increment10-mobile;
	@include tablet {
	  margin: 0 $increment10-tablet;
	}
	@include desktop {
	  margin: 0 $increment10--desktop;
	}
  }
    /***************************************************INCREMENT WIDTH******************************************/
  
	@mixin increment-0-width {
		width: $increment0-mobile;
		@include tablet {
		  width: $increment0-tablet;
		}
		@include deskwidth {
		  width: $increment0-desktop;
		}
	  }
	  
	  @mixin increment-1-width {
		width: $increment1-mobile;
		@include tablet {
		  width: $increment1-tablet;
		}
		@include desktop {
		  width: $increment1-desktop;
		}
	  }
	  
	  @mixin increment-2-width {
		width: $increment2-mobile;
		@include tablet {
		  width: $increment2-tablet;
		}
		@include desktop {
		  width: $increment2-desktop;
		}
	  }
	  
	  @mixin increment-3-width {
		width: $increment3-mobile;
		@include tablet {
		  width: $increment3-tablet;
		}
		@include desktop {
		  width: $increment3-desktop;
		}
	  }
	  
	  @mixin increment-4-width {
		width: $increment4-mobile;
		@include tablet {
		  width: $increment4-tablet;
		}
		@include desktop {
		  width: $increment4-desktop;
		}
	  }
	  
	  @mixin increment-5-width {
		width: $increment5-mobile;
		@include tablet {
		  width: $increment5-tablet;
		}
		@include desktop {
		  width: $increment5-desktop;
		}
	  }
	  
	  @mixin increment-6-width {
		width: $increment6-mobile;
		@include tablet {
		  width: $increment6-tablet;
		}
		@include desktop {
		  width: $increment6-desktop;
		}
	  }
	  
	  @mixin increment-7-width {
		width: $increment7-mobile;
		@include tablet {
		  width: $increment7-tablet;
		}
		@include desktop {
		  width: $increment7-desktop;
		}
	  }
	  
	  @mixin increment-8-width {
		width: $increment8-mobile;
		@include tablet {
		  width: $increment8-tablet;
		}
		@include desktop {
		  width: $increment8--desktop;
		}
	  }
	  
	  @mixin increment-9-width {
		width: $increment9-mobile;
		@include tablet {
		  width: $increment9-tablet;
		}
		@include desktop {
		  width: $increment9--desktop;
		}
	  }
	  
	  @mixin increment-10-width {
		width: $increment10-mobile;
		@include tablet {
		  width: $increment10-tablet;
		}
		@include desktop {
		  width: $increment10--desktop;
		}
	  }
  
  /***************************************************INCREMENT HEIGHT******************************************/
  
  @mixin increment-0-height {
	height: $increment0-mobile;
	@include tablet {
	  height: $increment0-tablet;
	}
	@include deskheight {
	  height: $increment0-desktop;
	}
  }
  
  @mixin increment-1-height {
	height: $increment1-mobile;
	@include tablet {
	  height: $increment1-tablet;
	}
	@include desktop {
	  height: $increment1-desktop;
	}
  }
  
  @mixin increment-2-height {
	height: $increment2-mobile;
	@include tablet {
	  height: $increment2-tablet;
	}
	@include desktop {
	  height: $increment2-desktop;
	}
  }
  
  @mixin increment-3-height {
	height: $increment3-mobile;
	@include tablet {
	  height: $increment3-tablet;
	}
	@include desktop {
	  height: $increment3-desktop;
	}
  }
  
  @mixin increment-4-height {
	height: $increment4-mobile;
	@include tablet {
	  height: $increment4-tablet;
	}
	@include desktop {
	  height: $increment4-desktop;
	}
  }
  
  @mixin increment-5-height {
	height: $increment5-mobile;
	@include tablet {
	  height: $increment5-tablet;
	}
	@include desktop {
	  height: $increment5-desktop;
	}
  }
  
  @mixin increment-6-height {
	height: $increment6-mobile;
	@include tablet {
	  height: $increment6-tablet;
	}
	@include desktop {
	  height: $increment6-desktop;
	}
  }
  
  @mixin increment-7-height {
	height: $increment7-mobile;
	@include tablet {
	  height: $increment7-tablet;
	}
	@include desktop {
	  height: $increment7-desktop;
	}
  }
  
  @mixin increment-8-height {
	height: $increment8-mobile;
	@include tablet {
	  height: $increment8-tablet;
	}
	@include desktop {
	  height: $increment8--desktop;
	}
  }
  
  @mixin increment-9-height {
	height: $increment9-mobile;
	@include tablet {
	  height: $increment9-tablet;
	}
	@include desktop {
	  height: $increment9--desktop;
	}
  }
  
  @mixin increment-10-height {
	height: $increment10-mobile;
	@include tablet {
	  height: $increment10-tablet;
	}
	@include desktop {
	  height: $increment10--desktop;
	}
  }
  
  /***************************************************OTHER COMMON INCREMENTS********************************/
  
  /*******TOP****/
  
  @mixin increment20px-top {
	margin-top: $increment20px-mobile;
	@include tablet {
	  margin-top: $increment20px-tablet;
	}
	@include desktop {
	  margin-top: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-top {
	margin-top: $increment54px-mobile;
	@include tablet {
	  margin-top: $increment54px-tablet;
	}
	@include desktop {
	  margin-top: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-top {
	margin-top: $increment68px-mobile;
	@include tablet {
	  margin-top: $increment68px-tablet;
	}
	@include desktop {
	  margin-top: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-top {
	margin-top: $increment100px-mobile;
	@include tablet {
	  margin-top: $increment100px-tablet;
	}
	@include desktop {
	  margin-top: $increment100px-desktop;
	}
  }
  
  /***************RIGHT********************/
  
  @mixin increment20px-right {
	margin-right: $increment20px-mobile;
	@include tablet {
	  margin-right: $increment20px-tablet;
	}
	@include desktop {
	  margin-right: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-right {
	margin-right: $increment54px-mobile;
	@include tablet {
	  margin-right: $increment54px-tablet;
	}
	@include desktop {
	  margin-right: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-right {
	margin-right: $increment68px-mobile;
	@include tablet {
	  margin-right: $increment68px-tablet;
	}
	@include desktop {
	  margin-right: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-right {
	margin-right: $increment100px-mobile;
	@include tablet {
	  margin-right: $increment100px-tablet;
	}
	@include desktop {
	  margin-right: $increment100px-desktop;
	}
  }
  
  /****************BOTTOM***************/
  
  @mixin increment20px-bottom {
	margin-bottom: $increment20px-mobile;
	@include tablet {
	  margin-bottom: $increment20px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-bottom {
	margin-bottom: $increment54px-mobile;
	@include tablet {
	  margin-bottom: $increment54px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-bottom {
	margin-bottom: $increment68px-mobile;
	@include tablet {
	  margin-bottom: $increment68px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-bottom {
	margin-bottom: $increment100px-mobile;
	@include tablet {
	  margin-bottom: $increment100px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment100px-desktop;
	}
  }
  
  /*****************LEFT****************/
  
  @mixin increment20px-left {
	margin-left: $increment20px-mobile;
	@include tablet {
	  margin-left: $increment20px-tablet;
	}
	@include desktop {
	  margin-left: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-left {
	margin-left: $increment54px-mobile;
	@include tablet {
	  margin-left: $increment54px-tablet;
	}
	@include desktop {
	  margin-left: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-left {
	margin-left: $increment68px-mobile;
	@include tablet {
	  margin-left: $increment68px-tablet;
	}
	@include desktop {
	  margin-left: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-left {
	margin-left: $increment100px-mobile;
	@include tablet {
	  margin-left: $increment100px-tablet;
	}
	@include desktop {
	  margin-left: $increment100px-desktop;
	}
  }
  
  /************VERTICAL***********/
  
  @mixin increment20px-vertical {
	margin: $increment20px-mobile 0;
	@include tablet {
	  margin: $increment20px-tablet 0;
	}
	@include desktop {
	  margin: $increment20px-desktop 0;
	}
  }
  
  @mixin increment54px-vertical {
	margin: $increment54px-mobile 0;
	@include tablet {
	  margin: $increment54px-tablet 0;
	}
	@include desktop {
	  margin: $increment54px-desktop 0;
	}
  }
  
  @mixin increment68px-vertical {
	margin: $increment68px-mobile 0;
	@include tablet {
	  margin: $increment68px-tablet 0;
	}
	@include desktop {
	  margin: $increment68px-desktop 0;
	}
  }
  
  @mixin increment100px-vertical {
	margin: $increment100px-mobile 0;
	@include tablet {
	  margin: $increment100px-tablet 0;
	}
	@include desktop {
	  margin: $increment100px-desktop 0;
	}
  }
  
  /**********HORIZONTAL*********/
  
  @mixin increment20px-horizontal {
	margin: 0 $increment20px-mobile;
	@include tablet {
	  margin: 0 $increment20px-tablet;
	}
	@include desktop {
	  margin: 0 $increment20px-desktop;
	}
  }
  
  @mixin increment54px-horizontal {
	margin: 0 $increment54px-mobile;
	@include tablet {
	  margin: 0 $increment54px-tablet;
	}
	@include desktop {
	  margin: 0 $increment54px-desktop;
	}
  }
  
  @mixin increment68px-horizontal {
	margin: 0 $increment68px-mobile;
	@include tablet {
	  margin: 0 $increment68px-tablet;
	}
	@include desktop {
	  margin: 0 $increment68px-desktop;
	}
  }
  
  @mixin increment100px-horizontal {
	margin: 0 $increment100px-mobile;
	@include tablet {
	  margin: 0 $increment100px-tablet;
	}
	@include desktop {
	  margin: 0 $increment100px-desktop;
	}
  }
  
 /***********WIDTH*********/
  
  @mixin increment20px-width {
	width: $increment20px-mobile;
	@include tablet {
	  width: $increment20px-tablet;
	}
	@include desktop {
	  width: $increment20px-desktop;
	}
  }

 @mixin increment54px-width {
	width: $increment54px-mobile;
	@include tablet {
	  width: $increment54px-tablet;
	}
	@include desktop {
	  width: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-width {
	width: $increment68px-mobile;
	@include tablet {
	  width: $increment68px-tablet;
	}
	@include desktop {
	  width: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-width {
	width: $increment100px-mobile;
	@include tablet {
	  width: $increment100px-tablet;
	}
	@include desktop {
	  width: $increment100px-desktop;
	}
  }

  /***********HEIGHT*********/
  @mixin increment20px-height {
	height: $increment20px-mobile;
	@include tablet {
	  height: $increment20px-tablet;
	}
	@include desktop {
	  height: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-height {
	height: $increment54px-mobile;
	@include tablet {
	  height: $increment54px-tablet;
	}
	@include desktop {
	  height: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-height {
	height: $increment68px-mobile;
	@include tablet {
	  height: $increment68px-tablet;
	}
	@include desktop {
	  height: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-height {
	height: $increment100px-mobile;
	@include tablet {
	  height: $increment100px-tablet;
	}
	@include desktop {
	  height: $increment100px-desktop;
	}
  }
  
  /**********************************************MOCKUP COLUMNS PADDING****************************************************************/
  
  @mixin mockup-column-padding-outside-right {
	padding-right: $mockup-column-outside-mobile;
	@include tablet {
	  padding-right: $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-right {
	padding-right: $mockup-column-small-mobile;
	@include tablet {
	  padding-right: $mockup-column-small-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-right {
	padding-right: $mockup-column-large-mobile;
	@include tablet {
	  padding-right: $mockup-column-large-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-large-desktop;
	}
  }
  
  /***LEFT*******/
  
  @mixin mockup-column-padding-outside-left {
	padding-left: $mockup-column-outside-mobile;
	@include tablet {
	  padding-left: $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-left {
	padding-left: $mockup-column-small-mobile;
	@include tablet {
	  padding-left: $mockup-column-small-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-left {
	padding-left: $mockup-column-large-mobile;
	@include tablet {
	  padding-left: $mockup-column-large-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-large-desktop;
	}
  }
  
  /***HORIZONTAL*******/
  
  @mixin mockup-column-padding-outside-horizontal {
	padding: 0 $mockup-column-outside-mobile;
	@include tablet {
	  padding: 0 $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-horizontal {
	padding: 0 $mockup-column-small-mobile;
	@include tablet {
	  padding: 0 $mockup-column-small-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-horizontal {
	padding: 0 $mockup-column-large-mobile;
	@include tablet {
	  padding: 0 $mockup-column-large-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-large-desktop;
	}
  }
  /*************************************TEXT STYLES************************************/
@use "./colors" as *;
@use "./breakpoints" as *;
@use "./variables" as *;
/*****************************************H1-Main-heading***************************/

@mixin h1-main-header {
    font-family: AvenirNextLTProBold;
    font-size: 1.5rem; //24px 
    line-height: 2.0rem; //32px
    color: $white;
    @include tablet {
        font-size: 2.3rem; //36px 2.25rem 
        line-height: 2.8rem; //44px
    }
    @include desktop {
        font-size: 2.3rem; //36px 2.25rem 
        line-height: 2.8rem; //44px
    }
}

@mixin h1-main-header-regular {
    @include h1-main-header;
    font-family: AvenirNextLTProRegular;
}

@mixin h1-main-header-black {
    @include h1-main-header;
    color: $black;
}

@mixin h1-desktop-size-that-should-be-available-on-mobile {
    color: $white;
    font-family: AvenirNextLTProBold;
    font-size: 8.0vw;
    line-height: 10.8vw;
}


/*********************H2 - Header subheader for mobile*********************/

@mixin h2-header {
    font-size: 0.9rem; //14px 
    line-height: 1.4rem; //22px 
    font-family: AvenirNextLTProDemi;
    color: $black;
    @include tablet {
        font-size: 1.5rem; //24px 1.5rem 
        line-height: 2.0; //32px 2rem
        font-family: AvenirNextLTProBold;
    }
    @include desktop {
        font-size: 1.5rem; //24px 1.5rem 
        line-height: 2.0rem; //32px 2rem 
    }
}

@mixin h2-header-white {
    @include h2-header;
    color: $white;
}

@mixin h2-header-responsive-white {
    @include h2-header-responsive;
    color: $white;
}

@mixin h2-desktop-size-that-should-be-available-on-mobile {
    color: $white;
    font-size: 1.5rem; //24px 1.5rem 
    line-height: 2.0; //32px 2rem
    font-family: AvenirNextLTProBold;
}


/*******************************H3 - Subheader for Tablet and Desktop************/

@mixin h3-subheader {
    font-size: 1.0rem; //16px 
    line-height: 1.5rem; // 24px
    font-family: AvenirNextLTProDemi;
    color: $black;
    @include tablet {
        font-size: 1.0rem; //16px 
        line-height: 1.5rem; // 24px
    }
    @include desktop {
        font-size: 1.0rem; //16px 
        line-height: 1.5rem; // 24px
    }
}

@mixin h3-subheader-white {
    @include h3-subheader;
    color: $white;
}

@mixin h3-subheader-bold {
    @include h3-subheader;
    font-family: AvenirNextLTProBold;
}

@mixin h3-subheader-white-bold {
    @include h3-subheader;
    font-family: AvenirNextLTProBold;
    color: $white;
}


/********************H4 Paragraph & Comments******************************/

@mixin h4-paragraphs-and-comments {
    font-family: AvenirNextLTProRegular;
    color: $black;
    font-size: 0.8rem; // 13px
    line-height: 1.1rem; //18px
    @include tablet {
        font-size: 0.9rem; // 14px
        line-height: 1.4rem; //22px
    }
    @include desktop {
        font-size: 0.9rem; // 14px
        line-height: 1.4rem; // 22px 
    }
}

@mixin h4-paragraphs-and-comments-bold {
    @include h4-paragraphs-and-comments;
    font-family: AvenirNextLTProBold;
}

@mixin h4-paragraphs-and-comments-white {
    @include h4-paragraphs-and-comments;
    color: $white;
}

@mixin h4-paragraphs-and-comments-white-bold {
    @include h4-paragraphs-and-comments-bold;
    color: $white;
}

@mixin h4-paragraphs-and-comments-white-demi {
    @include h4-paragraphs-and-comments-white;
    font-family: AvenirNextLTProDemi;
    color: $white;
}

$time: 330ms;
$btns: ( 
alpha: ( gstart: #21D4FD,
gend: #B721FF,
tstart: #B721FF,
tend: #21D4FD),
beta: ( gstart: #08AEEA,
gend: #2AF598,
tstart: #2AF598,
tend: #08AEEA),
gamma: ( gstart: #FEE140,
gend: #FA709A,
tstart: #FA709A,
tend: #FEE140),
delta: ( gstart: #3EECAC,
gend: #EE74E1,
tstart: #EE74E1,
tend: #3EECAC));
.btn {
	outline: none;;
	margin-right: 30px;
	margin-top: 30px;
	font-family: 'Source Sans Pro', sans-serif;
	font-weight: 900;
	padding: 1.25rem 2rem;
	font-size: 1rem;
	border-radius: 3.5rem / 100%;
	position: relative;
	min-width: 15rem;
	max-width: 90vw;
	overflow: hidden;
	border: 0;
	cursor: pointer;
	text-transform: uppercase;
	letter-spacing: 0.05em;
	transition: all $time;
	outline: none;;
	&+& {
		outline: none;;
		margin-top: 1rem;
	}
	span {
		outline: none;;
		position: relative;
		z-index: 1;
	}
	&:before {
		outline: none;;
		content: "";
		background-color: #21D4FD;
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		transform: scale(4) translateX(-100%);
		transition: all $time * 1.5 ease-out;
		outline: none;;
	}
	&:hover,
	&:focus,
	&:active {
		&:before {
			outline: none;;
			transform: scale(4) translate(37%);
		}
	}
}

@each $bname,
$bcolors in $btns {
	.btn--#{$bname} {
		outline: none;;
		background-color: map-get($bcolors, gstart);
		color: map-get($bcolors, tstart);
		&:before {
			background-color: map-get($bcolors, gend);
			background-image: linear-gradient(to right, map-get($bcolors, gend) 30%, map-get($bcolors, gstart) 100%);
			position: absolute;
		}
		&:hover,
		&:focus,
		&:active {
			outline: none;;
			color: map-get($bcolors, tend);
		}
	}
}

.btn-form {
	outline: none;;
	outline: none;;
	margin-right: 30px;
	margin-top: 30px;
	font-family: 'Source Sans Pro', sans-serif;
	font-weight: 900;
	padding: 1.25rem 2rem;
	font-size: 1rem;
	border-radius: 3.5rem / 100%;
	position: relative;
	min-width: 15rem;
	max-width: 90vw;
	overflow: hidden;
	border: 0;
	cursor: pointer;
	text-transform: uppercase;
	letter-spacing: 0.05em;
	transition: all $time;
	 ;
	&+& {
		margin-top: 1rem;
		 ;
	}
	span {
		position: relative;
		z-index: 1;
		 ;
	}
	&:before {
		content: "";
		background-color: #21D4FD;
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		transform: scale(4) translateX(-100%);
		transition: all $time * 1.5 ease-out;
		 ;
	}
	&:hover,
	&:focus,
	&:active {
		&:before {
			 
			//transform: scale(4) translate(37%);
		  //  color: map-get($bcolors, tstart);
		  background-color:#B721FF;;
		}
	}
}

@each $bname,
$bcolors in $btns {
	.btn-form--#{$bname} {
		background-color: map-get($bcolors, gstart);
		color: map-get($bcolors, tstart);
		&:before {
			background-color: map-get($bcolors, gend);
			background-image: linear-gradient(to right, map-get($bcolors, gend) 30%, map-get($bcolors, gstart) 100%);
			position: absolute;
		}
		&:hover,
		&:focus,
		&:active {
		//	color: map-get($bcolors, tstart);
		color: #B721FF;
		}
	}
}
/********************************h4-lists-and-contacts-information*************/

@mixin h4-list-contact-information {
    color: $black;
    font-family: AvenirNextLTProRegular;
    font-size: 0.8rem; //13px 
    line-height: 1.1rem; //18px 
    @include tablet {
        font-size: 0.9rem; //14px 
        line-height: 1.3rem; //20px
    }
    @include desktop {
        font-size: 0.9rem; //14px 
        line-height: 1.3rem; //20px 
    }
}

@mixin h4-list-contact-information-bold {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProBold;
}

@mixin h4-list-contact-information-demi {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProDemi;
}

@mixin h4-list-contact-information-white {
    @include h4-list-contact-information;
    color: $white;
}

@mixin h4-list-contact-information-white-demi {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProDemi;
    color: $white;
}

@mixin h4-list-contact-information-white-bold {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProBold;
    color: $white;
}


//********************************P-Paragraph****************************/

@mixin p-paragraph {
    font-family: AvenirNextLTProRegular;
    color: $black;
    font-size: 0.8rem; //13px
    line-height: 1.1rem; //18px
    @include tablet {
        font-size: 0.9rem; //13px 
        line-height: 1.3rem; //18px 
        display: inline-block;
    }
    @include desktop {
        font-size: 0.9rem; //13px 
        line-height: 1.3rem; //18px
        display: inline-block;
    }
}

@mixin p-paragraph-bold {
    @include p-paragraph;
    font-family: AvenirNextLTProBold;
}

@mixin p-paragraph-white {
    @include p-paragraph;
    color: white;
}

@mixin p-paragraph-white-bold {
    @include p-paragraph-bold;
    color: white;
}

@mixin h1-page-header {
	font-family: TitilliumWebSemiBold600;
	color: $black;
	font-size: 28px; //28px
	line-height: 36px; //36px
	@include tablet {
        font-size: 32px; //32px
        line-height: 40px; //40px
	}
	@include desktop {
        font-size: 32px; //32px
        line-height: 40px; //40px
	}
}

@mixin h2-subheader {
	font-family: TitilliumWebSemiBold600;
	color: $black;
	font-size: 20px; //20px
	line-height: 28px; //28px
	@include tablet {
        font-size: 24px; //24px
        line-height: 32px; //28px
	}
	@include desktop {
        font-size: 24px; //24px
        line-height: 32px; //28px
	}
}
@mixin h3-labels-links-buttons {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 13px; //13px
	line-height: 20px; //20px
	@include tablet {
        font-size: 14px; //14px
        line-height: 22px; //22px
	}
	@include desktop {
        font-size: 14px; //14px
        line-height: 22px; //22px
	}
}
@mixin h4-labels-table-header {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 11px; //11px
	line-height: 16px; //16px
	@include tablet {
        font-size: 12px; //12px
        line-height: 18px; //18px
	}
	@include desktop {
        font-size: 12px; //12px
        line-height: 18px; //18px
	}
}
@mixin p1-body-large {
	font-family: TitilliumWebRegular400;
	color: $black;
    font-size: 15px; //15px
    line-height: 26px; //26px
	@include tablet {
        font-size: 16px; //12px
        line-height: 28px; //18px
	}
	@include desktop {
        font-size: 16px; //12px
        line-height: 28px; //18px
	}
}
@mixin p2-body-medium {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 16px; //13px
	line-height: 28px; //20px
	@include tablet {
        font-size: 13px; //13px
        line-height: 20px; //20px
	}
	@include desktop {
        font-size: 13px; //13px
        line-height: 20px; //20px
	}
}
@mixin p2-body-small {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 11px; //13px
	line-height: 16px; //20px
	@include tablet {
        font-size: 12px; //13px
        line-height: 18px; //20px
	}
	@include desktop {
        font-size: 12px; //13px
        line-height: 18px; //20px
	}
}

/*******************************************H5-Labels***********************/

@mixin h5-labels {
    font-size: 0.8rem; //12px
    line-height: 1.3rem; //20px 
    font-family: AvenirNextLTProRegular;
    color: $silver; //labels
    @include tablet {
        font-size: 0.8rem; //12px
        line-height: 1.3rem; // 20px
        font-family: AvenirNextLTProDemi;
    }
    @include desktop {
        font-size: 0.8rem; //12px
        line-height: 1.8rem; // 20px
    }
}

@mixin h5-labels-demi {
    @include h5-labels;
    font-family: AvenirNextLTProDemi;
}

@mixin h5-labels-bold {
    @include h5-labels;
    font-family: AvenirNextLTProBold;
}


//Breakpoints
@mixin tablet {
    @media (min-width: $breakpoint-tablet) {
        @content;
    }
}

@mixin desktop {
    @media (min-width: $breakpoint-desktop) {
        @content;
    }
}

//Text Styles- Desktop

@mixin d-header {
    font-family: $font-family;
    font-size: 1.875rem;
    line-height: 2.25rem;
    font-weight: 700;
	color: $text-black;
}

@mixin d-subheader {
	font-family: $font-family;
    font-size: 1.875rem;
    line-height: 2.25rem;
    font-weight: 500;
	color: $text-black;
}

@mixin d-subheader-grey {
	font-family: $font-family;
    font-size: 1.75rem;
    line-height: 2rem;
	color: $text-grey;
    font-weight: 500;
}

@mixin d-paragraph-bold {
	font-family: $font-family;
    font-size: 1.125rem;
    line-height: 1.25rem;
	color: $text-black;
    font-weight: 700;
}

@mixin d-paragraph-large {
	font-family: $font-family;
    font-size: 1.375rem;
    line-height: 1.4rem;
	color: $text-grey;
    font-weight: 400;
}

@mixin d-paragraph-medium {
	font-family: $font-family;
    font-size: 1.25rem;
    line-height: 1.4rem;
	color: $text-grey;
    font-weight: 400;
}

@mixin d-paragraph-small {
	font-family: $font-family;
    font-size: 1.4rem;
    line-height: 1rem;
	color: $text-grey;
    font-weight: 400;
}

//Button Styling
@mixin button-blue {
    background-color: $secondary-color-indigo;
    border-radius: 5px;
    color: white;
    padding: 0.875rem 1.813rem;
    appearance: none;
    border: none;
}

//Component Styling

@mixin component-styling {
    box-shadow: 0px 10px 25px rgba(19, 24, 44, 0.1);
    border-radius: 10px;
    padding: 3.75rem;
}
/***************************************STANDARD PADDING**************************************************/
@mixin standard-padding-right {
	padding-right: $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding-right: $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding-right: $mockup-column-outside-desktop; //80px right 80px left
	}
  }

  @mixin standard-padding-left {
	padding-left: $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding-left: $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding-left: $mockup-column-outside-desktop; //80px right 80px left
	}
  }
  
  @mixin standard-padding-horizontal {
	padding: 0 $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding: 0 $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding: 0 $mockup-column-outside-desktop; //80px right 80px left
	}
  }
  
  
  /**********************************WIDTH*100%*PADDING**********************************************/
  
  @mixin width-100-percent-minus-standard-padding-horizontal {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent} * 2));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent} * 2));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent} * 2));
	}
  }
  
  @mixin width-100-percent-minus-standard-padding-left {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent}));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent}));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent}));
	}
  }
  
  @mixin width-100-percent-minus-standard-padding-right {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent}));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent}));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent}));
	}
  }
  

  /*******************WIDTH MINUS MOCKUP COLUMN************/
  @mixin width-100-percent-minus-mockup-column-outside {
	width: calc(100% - #{$mockup-column-outside-mobile});
	@include tablet {
	  width: calc(100% - #{$mockup-column-outside-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-outside-desktop});
	}
  }
  @mixin width-100-percent-minus-mockup-column-outside-horizontal {
	width: calc(100% - (#{$mockup-column-outside-mobile} * 2));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet} *2));
	}
	@include desktop {
	  width: calc(100% - (#{mockup-column-outside-desktop} *2));
	}
  }
  @mixin width-100-percent-minus-mockup-column-small {
	width: calc(100% - #{mockup-column-small-mobile});
	@include tablet {
	  width: calc(100% -#{mockup-column-small-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-small-desktop});
	}
  }
  
  @mixin width-100-percent-minus-mockup-column-large {
	width: calc(100% - #{mockup-column-large-mobile});
	@include tablet {
	  width: calc(100% -#{mockup-column-large-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-large-desktop});
	}
  }
  



@mixin fancybutton {
  font-family: 'Source Sans Pro', sans-serif;
  font-weight: 900;
  padding: 1.25rem 2rem;
  font-size: 1rem;
  border-radius: 3.5rem / 100%;
  position: relative;
  min-width: 15rem;
  max-width: 90vw;
  overflow: hidden;
  border: 0;
  cursor: pointer;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  transition: all $time;
  
  & + & {
    margin-top: 1rem;
  }
  
  span {
    position: relative;
    z-index: 1;
  }
  
  &:before {
    content: "";
    background-color: #21D4FD;
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    transform: scale(4) translateX(-100%);
    transition: all $time * 1.5 ease-out;
  }
  
  &:hover,
  &:focus,
  &:active {    
    &:before {
      transform: scale(4) translate(37%);
    } 
  }
}


  /**********************************************DIVIDERS******************************************************/
  
  @mixin divider {
	background: $silver;
	width: 100%;
	height: 1px;
  }
  
  @mixin divider-form {
	background: $silver;
	width: 100%;
	height: 1px;
	margin-bottom: $increment2-px;
  }
  
  /**********************************************LOGOS******************************************************/
  
  @mixin logo-brainflix {
	img {
	  width: 120px; //120px
	  height: 24px; //24px
	  margin-left: 1rem;
	  @include tablet {
		margin-left: 2rem;
		width: 120px; //120px
		height: 24px; //24px
	  }
	  @include desktop {
		margin-left: 5rem;
	  }
	}
  }
  
  /**********************************************BUTTONS******************************************************/
  
  @mixin button {
	background-color: $brainflix-blue;
	@include h4-paragraphs-and-comments-white-demi;
	&:hover {
	  background: $brainflix-dark-blue-hover;
	}
	&:focus {
	  outline: none;
	}
	width: $increment28-px; //226px
	height: $increment6-px; 
	border-radius: $increment0-px; 
	border-color: $brainflix-blue;
	@include tablet {
	  padding: 0;
	  width: $increment20-px; //160px
	  height: $increment6-px; 
	}
  }
  
  /**********************************************MOCKUP COLUMNS****************************************************************/
  
  /*******************RIGHT*****************/
  
  @mixin mockup-column-outside-right {
	margin-right: $mockup-column-outside-mobile;
	@include tablet {
	  margin-right: $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-right {
	margin-right: $mockup-column-small-mobile;
	@include tablet {
	  margin-right: $mockup-column-small-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-right {
	margin-right: $mockup-column-large-mobile;
	@include tablet {
	  margin-right: $mockup-column-large-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-large-desktop;
	}
  }
  
  /*******************LEFT***************/
  
  @mixin mockup-column-outside-left {
	margin-left: $mockup-column-outside-mobile;
	@include tablet {
	  margin-left: $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-left {
	margin-left: $mockup-column-small-mobile;
	@include tablet {
	  margin-left: $mockup-column-small-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-left {
	margin-left: $mockup-column-large-mobile;
	@include tablet {
	  margin-left: $mockup-column-large-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-large-desktop;
	}
  }
  
  /*********************HORIZONTAL**************/
  
  @mixin mockup-column-outside-horizontal {
	margin: 0 $mockup-column-outside-mobile;
	@include tablet {
	  margin: 0 $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-horizontal {
	margin: 0 $mockup-column-small-mobile;
	@include tablet {
	  margin: 0 $mockup-column-small-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-horizontal {
	margin: 0 $mockup-column-large-mobile;
	@include tablet {
	  margin: 0 $mockup-column-large-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-large-desktop;
	}
  }
  
  /*********************************************WIDTH 100% MINUS INCREMENTS*********************************/
  
  @mixin width-100-percent-minus-increment0 {
	width: calc(100% - #{increment0-mobile});
	@include tablet {
	  width: calc(100% -#{increment0-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment0-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment1 {
	width: calc(100% - #{increment1-mobile});
	@include tablet {
	  width: calc(100% - #{increment1-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment1-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment2 {
	width: calc(100% - #{increment2-mobile});
	@include tablet {
	  width: calc(100% -#{increment2-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment2-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment3 {
	width: calc(100% - #{increment3-mobile});
	@include tablet {
	  width: calc(100% -#{increment3-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment3-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment4 {
	width: calc(100% - #{increment4-mobile});
	@include tablet {
	  width: calc(100% -#{increment4-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment4-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment5 {
	width: calc(100% - #{increment5-mobile});
	@include tablet {
	  width: calc(100% -#{increment5-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment5-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment6 {
	width: calc(100% - #{increment6-mobile});
	@include tablet {
	  width: calc(100% -#{increment6-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment6-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment7 {
	width: calc(100% - #{increment7-mobile});
	@include tablet {
	  width: calc(100% -#{increment7-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment7-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment8 {
	width: calc(100% - #{increment8-mobile});
	@include tablet {
	  width: calc(100% -#{increment8-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment8-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment9 {
	width: calc(100% - #{increment9-mobile});
	@include tablet {
	  width: calc(100% -#{increment9-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment9-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment10 {
	width: calc(100% - #{increment10-mobile});
	@include tablet {
	  width: calc(100% -#{increment10-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment10-desktop});
	}
  }
  

  
  /************************************%%%%%%%%%%%%%%%%%%%%%******************************************************
  /******%%%%%%%%%%%%%%%%%%%%%%%****CONTENTS LIST:*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****TOP MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *****%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%******RIGHT MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***BOTTOM MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***LEFT MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ***********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*VERTICAL MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ***********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*HORIZTONAL**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***WIDTH **********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%****HEIGHT**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****OTHER COMMON INCREMENTS*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
   ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****MOCKUP COLUMN PADDING*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********/
  
  /*******************************************************INCREMENT MARGIN TOP******************************************/
  
  @mixin increment-0-top {
	margin-top: $increment0-mobile;
	@include tablet {
	  margin-top: $increment0-tablet;
	}
	@include desktop {
	  margin-top: $increment0-desktop;
	}
  }
  
  @mixin increment-1-top {
	margin-top: $increment1-mobile;
	@include tablet {
	  margin-top: $increment1-tablet;
	}
	@include desktop {
	  margin-top: $increment1-desktop;
	}
  }
  
  @mixin increment-2-top {
	margin-top: $increment2-mobile;
	@include tablet {
	  margin-top: $increment2-tablet;
	}
	@include desktop {
	  margin-top: $increment2-desktop;
	}
  }
  
  @mixin increment-3-top {
	margin-top: $increment3-mobile;
	@include tablet {
	  margin-top: $increment3-tablet;
	}
	@include desktop {
	  margin-top: $increment3-desktop;
	}
  }
  
  @mixin increment-4-top {
	margin-top: $increment4-mobile;
	@include tablet {
	  margin-top: $increment4-tablet;
	}
	@include desktop {
	  margin-top: $increment4-desktop;
	}
  }
  
  @mixin increment-5-top {
	margin-top: $increment5-mobile;
	@include tablet {
	  margin-top: $increment5-tablet;
	}
	@include desktop {
	  margin-top: $increment5-desktop;
	}
  }
  
  @mixin increment-6-top {
	margin-top: $increment6-mobile;
	@include tablet {
	  margin-top: $increment6-tablet;
	}
	@include desktop {
	  margin-top: $increment6-desktop;
	}
  }
  
  @mixin increment-7-top {
	margin-top: $increment7-mobile;
	@include tablet {
	  margin-top: $increment7-tablet;
	}
	@include desktop {
	  margin-top: $increment7-desktop;
	}
  }
  
  @mixin increment-8-top {
	margin-top: $increment8-mobile;
	@include tablet {
	  margin-top: $increment8-tablet;
	}
	@include desktop {
	  margin-top: $increment8--desktop;
	}
  }
  
  @mixin increment-9-top {
	margin-top: $increment9-mobile;
	@include tablet {
	  margin-top: $increment9-tablet;
	}
	@include desktop {
	  margin-top: $increment9--desktop;
	}
  }
  
  @mixin increment-10-top {
	margin-top: $increment10-mobile;
	@include tablet {
	  margin-top: $increment10-tablet;
	}
	@include desktop {
	  margin-top: $increment10--desktop;
	}
  }
  
  /***********************************INCREMENT RIGHT*************************************************/
  
  @mixin increment-0-right {
	margin-right: $increment0-mobile;
	@include tablet {
	  margin-right: $increment0-tablet;
	}
	@include desktop {
	  margin-right: $increment0-desktop;
	}
  }
  
  @mixin increment-1-right {
	margin-right: $increment1-mobile;
	@include tablet {
	  margin-right: $increment1-tablet;
	}
	@include desktop {
	  margin-right: $increment1-desktop;
	}
  }
  
  @mixin increment-2-right {
	margin-right: $increment2-mobile;
	@include tablet {
	  margin-right: $increment2-tablet;
	}
	@include desktop {
	  margin-right: $increment2-desktop;
	}
  }
  
  @mixin increment-3-right {
	margin-right: $increment3-mobile;
	@include tablet {
	  margin-right: $increment3-tablet;
	}
	@include desktop {
	  margin-right: $increment3-desktop;
	}
  }
  
  @mixin increment-4-right {
	margin-right: $increment4-mobile;
	@include tablet {
	  margin-right: $increment4-tablet;
	}
	@include desktop {
	  margin-right: $increment4-desktop;
	}
  }
  
  @mixin increment-5-right {
	margin-right: $increment5-mobile;
	@include tablet {
	  margin-right: $increment5-tablet;
	}
	@include desktop {
	  margin-right: $increment5-desktop;
	}
  }
  
  @mixin increment-6-right {
	margin-right: $increment6-mobile;
	@include tablet {
	  margin-right: $increment6-tablet;
	}
	@include desktop {
	  margin-right: $increment6-desktop;
	}
  }
  
  @mixin increment-7-right {
	margin-right: $increment7-mobile;
	@include tablet {
	  margin-right: $increment7-tablet;
	}
	@include desktop {
	  margin-right: $increment7-desktop;
	}
  }
  
  @mixin increment-8-right {
	margin-right: $increment8-mobile;
	@include tablet {
	  margin-right: $increment8-tablet;
	}
	@include desktop {
	  margin-right: $increment8--desktop;
	}
  }
  
  @mixin increment-9-right {
	margin-right: $increment9-mobile;
	@include tablet {
	  margin-right: $increment9-tablet;
	}
	@include desktop {
	  margin-right: $increment9--desktop;
	}
  }
  
  @mixin increment-10-right {
	margin-right: $increment10-mobile;
	@include tablet {
	  margin-right: $increment10-tablet;
	}
	@include desktop {
	  margin-right: $increment10--desktop;
	}
  }
  
  /*******************************************************INCREMENT MARGIN BOTTOM******************************************/
  
  @mixin increment-0-bottom {
	margin-bottom: $increment0-mobile;
	@include tablet {
	  margin-bottom: $increment0-tablet;
	}
	@include desktop {
	  margin-bottom: $increment0-desktop;
	}
  }
  
  @mixin increment-1-bottom {
	margin-bottom: $increment1-mobile;
	@include tablet {
	  margin-bottom: $increment1-tablet;
	}
	@include desktop {
	  margin-bottom: $increment1-desktop;
	}
  }
  
  @mixin increment-2-bottom {
	margin-bottom: $increment2-mobile;
	@include tablet {
	  margin-bottom: $increment2-tablet;
	}
	@include desktop {
	  margin-bottom: $increment2-desktop;
	}
  }
  
  @mixin increment-3-bottom {
	margin-bottom: $increment3-mobile;
	@include tablet {
	  margin-bottom: $increment3-tablet;
	}
	@include desktop {
	  margin-bottom: $increment3-desktop;
	}
  }
  
  @mixin increment-4-bottom {
	margin-bottom: $increment4-mobile;
	@include tablet {
	  margin-bottom: $increment4-tablet;
	}
	@include desktop {
	  margin-bottom: $increment4-desktop;
	}
  }
  
  @mixin increment-5-bottom {
	margin-bottom: $increment5-mobile;
	@include tablet {
	  margin-bottom: $increment5-tablet;
	}
	@include desktop {
	  margin-bottom: $increment5-desktop;
	}
  }
  
  @mixin increment-6-bottom {
	margin-bottom: $increment6-mobile;
	@include tablet {
	  margin-bottom: $increment6-tablet;
	}
	@include desktop {
	  margin-bottom: $increment6-desktop;
	}
  }
  
  @mixin increment-7-bottom {
	margin-bottom: $increment7-mobile;
	@include tablet {
	  margin-bottom: $increment7-tablet;
	}
	@include desktop {
	  margin-bottom: $increment7-desktop;
	}
  }
  
  @mixin increment-8-bottom {
	margin-bottom: $increment8-mobile;
	@include tablet {
	  margin-bottom: $increment8-tablet;
	}
	@include desktop {
	  margin-bottom: $increment8--desktop;
	}
  }
  
  @mixin increment-9-bottom {
	margin-bottom: $increment9-mobile;
	@include tablet {
	  margin-bottom: $increment9-tablet;
	}
	@include desktop {
	  margin-bottom: $increment9--desktop;
	}
  }
  
  @mixin increment-10-bottom {
	margin-bottom: $increment10-mobile;
	@include tablet {
	  margin-bottom: $increment10-tablet;
	}
	@include desktop {
	  margin-bottom: $increment10--desktop;
	}
  }
  
  /*******************************************************INCREMENT LEFT******************************************/
  
  @mixin increment-0-left {
	margin-left: $increment0-mobile;
	@include tablet {
	  margin-left: $increment0-tablet;
	}
	@include desktop {
	  margin-left: $increment0-desktop;
	}
  }
  
  @mixin increment-1-left {
	margin-left: $increment1-mobile;
	@include tablet {
	  margin-left: $increment1-tablet;
	}
	@include desktop {
	  margin-left: $increment1-desktop;
	}
  }
  
  @mixin increment-2-left {
	margin-left: $increment2-mobile;
	@include tablet {
	  margin-left: $increment2-tablet;
	}
	@include desktop {
	  margin-left: $increment2-desktop;
	}
  }
  
  @mixin increment-3-left {
	margin-left: $increment3-mobile;
	@include tablet {
	  margin-left: $increment3-tablet;
	}
	@include desktop {
	  margin-left: $increment3-desktop;
	}
  }
  
  @mixin increment-4-left {
	margin-left: $increment4-mobile;
	@include tablet {
	  margin-left: $increment4-tablet;
	}
	@include desktop {
	  margin-left: $increment4-desktop;
	}
  }
  
  @mixin increment-5-left {
	margin-left: $increment5-mobile;
	@include tablet {
	  margin-left: $increment5-tablet;
	}
	@include desktop {
	  margin-left: $increment5-desktop;
	}
  }
  
  @mixin increment-6-left {
	margin-left: $increment6-mobile;
	@include tablet {
	  margin-left: $increment6-tablet;
	}
	@include desktop {
	  margin-left: $increment6-desktop;
	}
  }
  
  @mixin increment-7-left {
	margin-left: $increment7-mobile;
	@include tablet {
	  margin-left: $increment7-tablet;
	}
	@include desktop {
	  margin-left: $increment7-desktop;
	}
  }
  
  @mixin increment-8-left {
	margin-left: $increment8-mobile;
	@include tablet {
	  margin-left: $increment8-tablet;
	}
	@include desktop {
	  margin-left: $increment8--desktop;
	}
  }
  
  @mixin increment-9-left {
	margin-left: $increment9-mobile;
	@include tablet {
	  margin-left: $increment9-tablet;
	}
	@include desktop {
	  margin-left: $increment9--desktop;
	}
  }
  
  @mixin increment-10-left {
	margin-left: $increment10-mobile;
	@include tablet {
	  margin-left: $increment10-tablet;
	}
	@include desktop {
	  margin-left: $increment10--desktop;
	}
  }
  
  /************************************VERTICAL MARGIN INCREMENTS******************************************************/
  
  @mixin increment-0-vertical {
	margin: $increment0-mobile 0;
	@include tablet {
	  margin: $increment0-tablet 0;
	}
	@include deskvertical {
	  margin: $increment0-desktop 0;
	}
  }
  
  @mixin increment-1-vertical {
	margin: $increment1-mobile 0;
	@include tablet {
	  margin: $increment1-tablet 0;
	}
	@include desktop {
	  margin: $increment1-desktop 0;
	}
  }
  
  @mixin increment-2-vertical {
	margin: $increment2-mobile 0;
	@include tablet {
	  margin: $increment2-tablet 0;
	}
	@include desktop {
	  margin: $increment2-desktop 0;
	}
  }
  
  @mixin increment-3-vertical {
	margin: $increment3-mobile 0;
	@include tablet {
	  margin: $increment3-tablet 0;
	}
	@include desktop {
	  margin: $increment3-desktop 0;
	}
  }
  
  @mixin increment-4-vertical {
	margin: $increment4-mobile 0;
	@include tablet {
	  margin: $increment4-tablet 0;
	}
	@include desktop {
	  margin: $increment4-desktop 0;
	}
  }
  
  @mixin increment-5-vertical {
	margin: $increment5-mobile 0;
	@include tablet {
	  margin: $increment5-tablet 0;
	}
	@include desktop {
	  margin: $increment5-desktop 0;
	}
  }
  
  @mixin increment-6-vertical {
	margin: $increment6-mobile 0;
	@include tablet {
	  margin: $increment6-tablet 0;
	}
	@include desktop {
	  margin: $increment6-desktop 0;
	}
  }
  
  @mixin increment-7-vertical {
	margin: $increment7-mobile 0;
	@include tablet {
	  margin: $increment7-tablet 0;
	}
	@include desktop {
	  margin: $increment7-desktop 0;
	}
  }
  
  @mixin increment-8-vertical {
	margin: $increment8-mobile 0;
	@include tablet {
	  margin: $increment8-tablet 0;
	}
	@include desktop {
	  margin: $increment8--desktop 0;
	}
  }
  
  @mixin increment-9-vertical {
	margin: $increment9-mobile 0;
	@include tablet {
	  margin: $increment9-tablet 0;
	}
	@include desktop {
	  margin: $increment9--desktop 0;
	}
  }
  
  @mixin increment-10-vertical {
	margin: $increment10-mobile 0;
	@include tablet {
	  margin: $increment10-tablet 0;
	}
	@include desktop {
	  margin: $increment10--desktop 0;
	}
  }
  
  /************************************HORIZONTAL MARGIN INCREMENTS******************************************************/
  
  @mixin increment-0-horizontal {
	margin: 0 $increment0-mobile;
	@include tablet {
	  margin: 0 $increment0-tablet;
	}
	@include deskhorizontal {
	  margin: 0 $increment0-desktop;
	}
  }
  
  @mixin increment-1-horizontal {
	margin: 0 $increment1-mobile;
	@include tablet {
	  margin: 0 $increment1-tablet;
	}
	@include desktop {
	  margin: 0 $increment1-desktop;
	}
  }
  
  @mixin increment-2-horizontal {
	margin: 0 $increment2-mobile;
	@include tablet {
	  margin: 0 $increment2-tablet;
	}
	@include desktop {
	  margin: 0 $increment2-desktop;
	}
  }
  
  @mixin increment-3-horizontal {
	margin: 0 $increment3-mobile;
	@include tablet {
	  margin: 0 $increment3-tablet;
	}
	@include desktop {
	  margin: 0 $increment3-desktop;
	}
  }
  
  @mixin increment-4-horizontal {
	margin: 0 $increment4-mobile;
	@include tablet {
	  margin: 0 $increment4-tablet;
	}
	@include desktop {
	  margin: 0 $increment4-desktop;
	}
  }
  
  @mixin increment-5-horizontal {
	margin: 0 $increment5-mobile;
	@include tablet {
	  margin: 0 $increment5-tablet;
	}
	@include desktop {
	  margin: 0 $increment5-desktop;
	}
  }
  
  @mixin increment-6-horizontal {
	margin: 0 $increment6-mobile;
	@include tablet {
	  margin: 0 $increment6-tablet;
	}
	@include desktop {
	  margin: 0 $increment6-desktop;
	}
  }
  
  @mixin increment-7-horizontal {
	margin: 0 $increment7-mobile;
	@include tablet {
	  margin: 0 $increment7-tablet;
	}
	@include desktop {
	  margin: 0 $increment7-desktop;
	}
  }
  
  @mixin increment-8-horizontal {
	margin: 0 $increment8-mobile;
	@include tablet {
	  margin: 0 $increment8-tablet;
	}
	@include desktop {
	  margin: 0 $increment8--desktop;
	}
  }
  
  @mixin increment-9-horizontal {
	margin: 0 $increment9-mobile;
	@include tablet {
	  margin: 0 $increment9-tablet;
	}
	@include desktop {
	  margin: 0 $increment9--desktop;
	}
  }
  
  @mixin increment-10-horizontal {
	margin: 0 $increment10-mobile;
	@include tablet {
	  margin: 0 $increment10-tablet;
	}
	@include desktop {
	  margin: 0 $increment10--desktop;
	}
  }
    /***************************************************INCREMENT WIDTH******************************************/
  
	@mixin increment-0-width {
		width: $increment0-mobile;
		@include tablet {
		  width: $increment0-tablet;
		}
		@include deskwidth {
		  width: $increment0-desktop;
		}
	  }
	  
	  @mixin increment-1-width {
		width: $increment1-mobile;
		@include tablet {
		  width: $increment1-tablet;
		}
		@include desktop {
		  width: $increment1-desktop;
		}
	  }
	  
	  @mixin increment-2-width {
		width: $increment2-mobile;
		@include tablet {
		  width: $increment2-tablet;
		}
		@include desktop {
		  width: $increment2-desktop;
		}
	  }
	  
	  @mixin increment-3-width {
		width: $increment3-mobile;
		@include tablet {
		  width: $increment3-tablet;
		}
		@include desktop {
		  width: $increment3-desktop;
		}
	  }
	  
	  @mixin increment-4-width {
		width: $increment4-mobile;
		@include tablet {
		  width: $increment4-tablet;
		}
		@include desktop {
		  width: $increment4-desktop;
		}
	  }
	  
	  @mixin increment-5-width {
		width: $increment5-mobile;
		@include tablet {
		  width: $increment5-tablet;
		}
		@include desktop {
		  width: $increment5-desktop;
		}
	  }
	  
	  @mixin increment-6-width {
		width: $increment6-mobile;
		@include tablet {
		  width: $increment6-tablet;
		}
		@include desktop {
		  width: $increment6-desktop;
		}
	  }
	  
	  @mixin increment-7-width {
		width: $increment7-mobile;
		@include tablet {
		  width: $increment7-tablet;
		}
		@include desktop {
		  width: $increment7-desktop;
		}
	  }
	  
	  @mixin increment-8-width {
		width: $increment8-mobile;
		@include tablet {
		  width: $increment8-tablet;
		}
		@include desktop {
		  width: $increment8--desktop;
		}
	  }
	  
	  @mixin increment-9-width {
		width: $increment9-mobile;
		@include tablet {
		  width: $increment9-tablet;
		}
		@include desktop {
		  width: $increment9--desktop;
		}
	  }
	  
	  @mixin increment-10-width {
		width: $increment10-mobile;
		@include tablet {
		  width: $increment10-tablet;
		}
		@include desktop {
		  width: $increment10--desktop;
		}
	  }
  
  /***************************************************INCREMENT HEIGHT******************************************/
  
  @mixin increment-0-height {
	height: $increment0-mobile;
	@include tablet {
	  height: $increment0-tablet;
	}
	@include deskheight {
	  height: $increment0-desktop;
	}
  }
  
  @mixin increment-1-height {
	height: $increment1-mobile;
	@include tablet {
	  height: $increment1-tablet;
	}
	@include desktop {
	  height: $increment1-desktop;
	}
  }
  
  @mixin increment-2-height {
	height: $increment2-mobile;
	@include tablet {
	  height: $increment2-tablet;
	}
	@include desktop {
	  height: $increment2-desktop;
	}
  }
  
  @mixin increment-3-height {
	height: $increment3-mobile;
	@include tablet {
	  height: $increment3-tablet;
	}
	@include desktop {
	  height: $increment3-desktop;
	}
  }
  
  @mixin increment-4-height {
	height: $increment4-mobile;
	@include tablet {
	  height: $increment4-tablet;
	}
	@include desktop {
	  height: $increment4-desktop;
	}
  }
  
  @mixin increment-5-height {
	height: $increment5-mobile;
	@include tablet {
	  height: $increment5-tablet;
	}
	@include desktop {
	  height: $increment5-desktop;
	}
  }
  
  @mixin increment-6-height {
	height: $increment6-mobile;
	@include tablet {
	  height: $increment6-tablet;
	}
	@include desktop {
	  height: $increment6-desktop;
	}
  }
  
  @mixin increment-7-height {
	height: $increment7-mobile;
	@include tablet {
	  height: $increment7-tablet;
	}
	@include desktop {
	  height: $increment7-desktop;
	}
  }
  
  @mixin increment-8-height {
	height: $increment8-mobile;
	@include tablet {
	  height: $increment8-tablet;
	}
	@include desktop {
	  height: $increment8--desktop;
	}
  }
  
  @mixin increment-9-height {
	height: $increment9-mobile;
	@include tablet {
	  height: $increment9-tablet;
	}
	@include desktop {
	  height: $increment9--desktop;
	}
  }
  
  @mixin increment-10-height {
	height: $increment10-mobile;
	@include tablet {
	  height: $increment10-tablet;
	}
	@include desktop {
	  height: $increment10--desktop;
	}
  }
  
  /***************************************************OTHER COMMON INCREMENTS********************************/
  
  /*******TOP****/
  
  @mixin increment20px-top {
	margin-top: $increment20px-mobile;
	@include tablet {
	  margin-top: $increment20px-tablet;
	}
	@include desktop {
	  margin-top: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-top {
	margin-top: $increment54px-mobile;
	@include tablet {
	  margin-top: $increment54px-tablet;
	}
	@include desktop {
	  margin-top: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-top {
	margin-top: $increment68px-mobile;
	@include tablet {
	  margin-top: $increment68px-tablet;
	}
	@include desktop {
	  margin-top: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-top {
	margin-top: $increment100px-mobile;
	@include tablet {
	  margin-top: $increment100px-tablet;
	}
	@include desktop {
	  margin-top: $increment100px-desktop;
	}
  }
  
  /***************RIGHT********************/
  
  @mixin increment20px-right {
	margin-right: $increment20px-mobile;
	@include tablet {
	  margin-right: $increment20px-tablet;
	}
	@include desktop {
	  margin-right: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-right {
	margin-right: $increment54px-mobile;
	@include tablet {
	  margin-right: $increment54px-tablet;
	}
	@include desktop {
	  margin-right: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-right {
	margin-right: $increment68px-mobile;
	@include tablet {
	  margin-right: $increment68px-tablet;
	}
	@include desktop {
	  margin-right: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-right {
	margin-right: $increment100px-mobile;
	@include tablet {
	  margin-right: $increment100px-tablet;
	}
	@include desktop {
	  margin-right: $increment100px-desktop;
	}
  }
  
  /****************BOTTOM***************/
  
  @mixin increment20px-bottom {
	margin-bottom: $increment20px-mobile;
	@include tablet {
	  margin-bottom: $increment20px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-bottom {
	margin-bottom: $increment54px-mobile;
	@include tablet {
	  margin-bottom: $increment54px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-bottom {
	margin-bottom: $increment68px-mobile;
	@include tablet {
	  margin-bottom: $increment68px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-bottom {
	margin-bottom: $increment100px-mobile;
	@include tablet {
	  margin-bottom: $increment100px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment100px-desktop;
	}
  }
  
  /*****************LEFT****************/
  
  @mixin increment20px-left {
	margin-left: $increment20px-mobile;
	@include tablet {
	  margin-left: $increment20px-tablet;
	}
	@include desktop {
	  margin-left: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-left {
	margin-left: $increment54px-mobile;
	@include tablet {
	  margin-left: $increment54px-tablet;
	}
	@include desktop {
	  margin-left: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-left {
	margin-left: $increment68px-mobile;
	@include tablet {
	  margin-left: $increment68px-tablet;
	}
	@include desktop {
	  margin-left: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-left {
	margin-left: $increment100px-mobile;
	@include tablet {
	  margin-left: $increment100px-tablet;
	}
	@include desktop {
	  margin-left: $increment100px-desktop;
	}
  }
  
  /************VERTICAL***********/
  
  @mixin increment20px-vertical {
	margin: $increment20px-mobile 0;
	@include tablet {
	  margin: $increment20px-tablet 0;
	}
	@include desktop {
	  margin: $increment20px-desktop 0;
	}
  }
  
  @mixin increment54px-vertical {
	margin: $increment54px-mobile 0;
	@include tablet {
	  margin: $increment54px-tablet 0;
	}
	@include desktop {
	  margin: $increment54px-desktop 0;
	}
  }
  
  @mixin increment68px-vertical {
	margin: $increment68px-mobile 0;
	@include tablet {
	  margin: $increment68px-tablet 0;
	}
	@include desktop {
	  margin: $increment68px-desktop 0;
	}
  }
  
  @mixin increment100px-vertical {
	margin: $increment100px-mobile 0;
	@include tablet {
	  margin: $increment100px-tablet 0;
	}
	@include desktop {
	  margin: $increment100px-desktop 0;
	}
  }
  
  /**********HORIZONTAL*********/
  
  @mixin increment20px-horizontal {
	margin: 0 $increment20px-mobile;
	@include tablet {
	  margin: 0 $increment20px-tablet;
	}
	@include desktop {
	  margin: 0 $increment20px-desktop;
	}
  }
  
  @mixin increment54px-horizontal {
	margin: 0 $increment54px-mobile;
	@include tablet {
	  margin: 0 $increment54px-tablet;
	}
	@include desktop {
	  margin: 0 $increment54px-desktop;
	}
  }
  
  @mixin increment68px-horizontal {
	margin: 0 $increment68px-mobile;
	@include tablet {
	  margin: 0 $increment68px-tablet;
	}
	@include desktop {
	  margin: 0 $increment68px-desktop;
	}
  }
  
  @mixin increment100px-horizontal {
	margin: 0 $increment100px-mobile;
	@include tablet {
	  margin: 0 $increment100px-tablet;
	}
	@include desktop {
	  margin: 0 $increment100px-desktop;
	}
  }
  
 /***********WIDTH*********/
  
  @mixin increment20px-width {
	width: $increment20px-mobile;
	@include tablet {
	  width: $increment20px-tablet;
	}
	@include desktop {
	  width: $increment20px-desktop;
	}
  }

 @mixin increment54px-width {
	width: $increment54px-mobile;
	@include tablet {
	  width: $increment54px-tablet;
	}
	@include desktop {
	  width: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-width {
	width: $increment68px-mobile;
	@include tablet {
	  width: $increment68px-tablet;
	}
	@include desktop {
	  width: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-width {
	width: $increment100px-mobile;
	@include tablet {
	  width: $increment100px-tablet;
	}
	@include desktop {
	  width: $increment100px-desktop;
	}
  }

  /***********HEIGHT*********/
  @mixin increment20px-height {
	height: $increment20px-mobile;
	@include tablet {
	  height: $increment20px-tablet;
	}
	@include desktop {
	  height: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-height {
	height: $increment54px-mobile;
	@include tablet {
	  height: $increment54px-tablet;
	}
	@include desktop {
	  height: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-height {
	height: $increment68px-mobile;
	@include tablet {
	  height: $increment68px-tablet;
	}
	@include desktop {
	  height: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-height {
	height: $increment100px-mobile;
	@include tablet {
	  height: $increment100px-tablet;
	}
	@include desktop {
	  height: $increment100px-desktop;
	}
  }
  
  /**********************************************MOCKUP COLUMNS PADDING****************************************************************/
  
  @mixin mockup-column-padding-outside-right {
	padding-right: $mockup-column-outside-mobile;
	@include tablet {
	  padding-right: $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-right {
	padding-right: $mockup-column-small-mobile;
	@include tablet {
	  padding-right: $mockup-column-small-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-right {
	padding-right: $mockup-column-large-mobile;
	@include tablet {
	  padding-right: $mockup-column-large-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-large-desktop;
	}
  }
  
  /***LEFT*******/
  
  @mixin mockup-column-padding-outside-left {
	padding-left: $mockup-column-outside-mobile;
	@include tablet {
	  padding-left: $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-left {
	padding-left: $mockup-column-small-mobile;
	@include tablet {
	  padding-left: $mockup-column-small-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-left {
	padding-left: $mockup-column-large-mobile;
	@include tablet {
	  padding-left: $mockup-column-large-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-large-desktop;
	}
  }
  
  /***HORIZONTAL*******/
  
  @mixin mockup-column-padding-outside-horizontal {
	padding: 0 $mockup-column-outside-mobile;
	@include tablet {
	  padding: 0 $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-horizontal {
	padding: 0 $mockup-column-small-mobile;
	@include tablet {
	  padding: 0 $mockup-column-small-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-horizontal {
	padding: 0 $mockup-column-large-mobile;
	@include tablet {
	  padding: 0 $mockup-column-large-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-large-desktop;
	}
  }
  /*************************************TEXT STYLES************************************/
@use "./colors" as *;
@use "./breakpoints" as *;
@use "./variables" as *;
/*****************************************H1-Main-heading***************************/

@mixin h1-main-header {
    font-family: AvenirNextLTProBold;
    font-size: 1.5rem; //24px 
    line-height: 2.0rem; //32px
    color: $white;
    @include tablet {
        font-size: 2.3rem; //36px 2.25rem 
        line-height: 2.8rem; //44px
    }
    @include desktop {
        font-size: 2.3rem; //36px 2.25rem 
        line-height: 2.8rem; //44px
    }
}

@mixin h1-main-header-regular {
    @include h1-main-header;
    font-family: AvenirNextLTProRegular;
}

@mixin h1-main-header-black {
    @include h1-main-header;
    color: $black;
}

@mixin h1-desktop-size-that-should-be-available-on-mobile {
    color: $white;
    font-family: AvenirNextLTProBold;
    font-size: 8.0vw;
    line-height: 10.8vw;
}


/*********************H2 - Header subheader for mobile*********************/

@mixin h2-header {
    font-size: 0.9rem; //14px 
    line-height: 1.4rem; //22px 
    font-family: AvenirNextLTProDemi;
    color: $black;
    @include tablet {
        font-size: 1.5rem; //24px 1.5rem 
        line-height: 2.0; //32px 2rem
        font-family: AvenirNextLTProBold;
    }
    @include desktop {
        font-size: 1.5rem; //24px 1.5rem 
        line-height: 2.0rem; //32px 2rem 
    }
}

@mixin h2-header-white {
    @include h2-header;
    color: $white;
}

@mixin h2-header-responsive-white {
    @include h2-header-responsive;
    color: $white;
}

@mixin h2-desktop-size-that-should-be-available-on-mobile {
    color: $white;
    font-size: 1.5rem; //24px 1.5rem 
    line-height: 2.0; //32px 2rem
    font-family: AvenirNextLTProBold;
}


/*******************************H3 - Subheader for Tablet and Desktop************/

@mixin h3-subheader {
    font-size: 1.0rem; //16px 
    line-height: 1.5rem; // 24px
    font-family: AvenirNextLTProDemi;
    color: $black;
    @include tablet {
        font-size: 1.0rem; //16px 
        line-height: 1.5rem; // 24px
    }
    @include desktop {
        font-size: 1.0rem; //16px 
        line-height: 1.5rem; // 24px
    }
}

@mixin h3-subheader-white {
    @include h3-subheader;
    color: $white;
}

@mixin h3-subheader-bold {
    @include h3-subheader;
    font-family: AvenirNextLTProBold;
}

@mixin h3-subheader-white-bold {
    @include h3-subheader;
    font-family: AvenirNextLTProBold;
    color: $white;
}


/********************H4 Paragraph & Comments******************************/

@mixin h4-paragraphs-and-comments {
    font-family: AvenirNextLTProRegular;
    color: $black;
    font-size: 0.8rem; // 13px
    line-height: 1.1rem; //18px
    @include tablet {
        font-size: 0.9rem; // 14px
        line-height: 1.4rem; //22px
    }
    @include desktop {
        font-size: 0.9rem; // 14px
        line-height: 1.4rem; // 22px 
    }
}

@mixin h4-paragraphs-and-comments-bold {
    @include h4-paragraphs-and-comments;
    font-family: AvenirNextLTProBold;
}

@mixin h4-paragraphs-and-comments-white {
    @include h4-paragraphs-and-comments;
    color: $white;
}

@mixin h4-paragraphs-and-comments-white-bold {
    @include h4-paragraphs-and-comments-bold;
    color: $white;
}

@mixin h4-paragraphs-and-comments-white-demi {
    @include h4-paragraphs-and-comments-white;
    font-family: AvenirNextLTProDemi;
    color: $white;
}

$time: 330ms;
$btns: ( 
alpha: ( gstart: #21D4FD,
gend: #B721FF,
tstart: #B721FF,
tend: #21D4FD),
beta: ( gstart: #08AEEA,
gend: #2AF598,
tstart: #2AF598,
tend: #08AEEA),
gamma: ( gstart: #FEE140,
gend: #FA709A,
tstart: #FA709A,
tend: #FEE140),
delta: ( gstart: #3EECAC,
gend: #EE74E1,
tstart: #EE74E1,
tend: #3EECAC));
.btn {
	outline: none;;
	margin-right: 30px;
	margin-top: 30px;
	font-family: 'Source Sans Pro', sans-serif;
	font-weight: 900;
	padding: 1.25rem 2rem;
	font-size: 1rem;
	border-radius: 3.5rem / 100%;
	position: relative;
	min-width: 15rem;
	max-width: 90vw;
	overflow: hidden;
	border: 0;
	cursor: pointer;
	text-transform: uppercase;
	letter-spacing: 0.05em;
	transition: all $time;
	outline: none;;
	&+& {
		outline: none;;
		margin-top: 1rem;
	}
	span {
		outline: none;;
		position: relative;
		z-index: 1;
	}
	&:before {
		outline: none;;
		content: "";
		background-color: #21D4FD;
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		transform: scale(4) translateX(-100%);
		transition: all $time * 1.5 ease-out;
		outline: none;;
	}
	&:hover,
	&:focus,
	&:active {
		&:before {
			outline: none;;
			transform: scale(4) translate(37%);
		}
	}
}

@each $bname,
$bcolors in $btns {
	.btn--#{$bname} {
		outline: none;;
		background-color: map-get($bcolors, gstart);
		color: map-get($bcolors, tstart);
		&:before {
			background-color: map-get($bcolors, gend);
			background-image: linear-gradient(to right, map-get($bcolors, gend) 30%, map-get($bcolors, gstart) 100%);
			position: absolute;
		}
		&:hover,
		&:focus,
		&:active {
			outline: none;;
			color: map-get($bcolors, tend);
		}
	}
}

.btn-form {
	outline: none;;
	outline: none;;
	margin-right: 30px;
	margin-top: 30px;
	font-family: 'Source Sans Pro', sans-serif;
	font-weight: 900;
	padding: 1.25rem 2rem;
	font-size: 1rem;
	border-radius: 3.5rem / 100%;
	position: relative;
	min-width: 15rem;
	max-width: 90vw;
	overflow: hidden;
	border: 0;
	cursor: pointer;
	text-transform: uppercase;
	letter-spacing: 0.05em;
	transition: all $time;
	 ;
	&+& {
		margin-top: 1rem;
		 ;
	}
	span {
		position: relative;
		z-index: 1;
		 ;
	}
	&:before {
		content: "";
		background-color: #21D4FD;
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		transform: scale(4) translateX(-100%);
		transition: all $time * 1.5 ease-out;
		 ;
	}
	&:hover,
	&:focus,
	&:active {
		&:before {
			 
			//transform: scale(4) translate(37%);
		  //  color: map-get($bcolors, tstart);
		  background-color:#B721FF;;
		}
	}
}

@each $bname,
$bcolors in $btns {
	.btn-form--#{$bname} {
		background-color: map-get($bcolors, gstart);
		color: map-get($bcolors, tstart);
		&:before {
			background-color: map-get($bcolors, gend);
			background-image: linear-gradient(to right, map-get($bcolors, gend) 30%, map-get($bcolors, gstart) 100%);
			position: absolute;
		}
		&:hover,
		&:focus,
		&:active {
		//	color: map-get($bcolors, tstart);
		color: #B721FF;
		}
	}
}
/********************************h4-lists-and-contacts-information*************/

@mixin h4-list-contact-information {
    color: $black;
    font-family: AvenirNextLTProRegular;
    font-size: 0.8rem; //13px 
    line-height: 1.1rem; //18px 
    @include tablet {
        font-size: 0.9rem; //14px 
        line-height: 1.3rem; //20px
    }
    @include desktop {
        font-size: 0.9rem; //14px 
        line-height: 1.3rem; //20px 
    }
}

@mixin h4-list-contact-information-bold {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProBold;
}

@mixin h4-list-contact-information-demi {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProDemi;
}

@mixin h4-list-contact-information-white {
    @include h4-list-contact-information;
    color: $white;
}

@mixin h4-list-contact-information-white-demi {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProDemi;
    color: $white;
}

@mixin h4-list-contact-information-white-bold {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProBold;
    color: $white;
}


//********************************P-Paragraph****************************/

@mixin p-paragraph {
    font-family: AvenirNextLTProRegular;
    color: $black;
    font-size: 0.8rem; //13px
    line-height: 1.1rem; //18px
    @include tablet {
        font-size: 0.9rem; //13px 
        line-height: 1.3rem; //18px 
        display: inline-block;
    }
    @include desktop {
        font-size: 0.9rem; //13px 
        line-height: 1.3rem; //18px
        display: inline-block;
    }
}

@mixin p-paragraph-bold {
    @include p-paragraph;
    font-family: AvenirNextLTProBold;
}

@mixin p-paragraph-white {
    @include p-paragraph;
    color: white;
}

@mixin p-paragraph-white-bold {
    @include p-paragraph-bold;
    color: white;
}

@mixin h1-page-header {
	font-family: TitilliumWebSemiBold600;
	color: $black;
	font-size: 28px; //28px
	line-height: 36px; //36px
	@include tablet {
        font-size: 32px; //32px
        line-height: 40px; //40px
	}
	@include desktop {
        font-size: 32px; //32px
        line-height: 40px; //40px
	}
}

@mixin h2-subheader {
	font-family: TitilliumWebSemiBold600;
	color: $black;
	font-size: 20px; //20px
	line-height: 28px; //28px
	@include tablet {
        font-size: 24px; //24px
        line-height: 32px; //28px
	}
	@include desktop {
        font-size: 24px; //24px
        line-height: 32px; //28px
	}
}
@mixin h3-labels-links-buttons {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 13px; //13px
	line-height: 20px; //20px
	@include tablet {
        font-size: 14px; //14px
        line-height: 22px; //22px
	}
	@include desktop {
        font-size: 14px; //14px
        line-height: 22px; //22px
	}
}
@mixin h4-labels-table-header {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 11px; //11px
	line-height: 16px; //16px
	@include tablet {
        font-size: 12px; //12px
        line-height: 18px; //18px
	}
	@include desktop {
        font-size: 12px; //12px
        line-height: 18px; //18px
	}
}
@mixin p1-body-large {
	font-family: TitilliumWebRegular400;
	color: $black;
    font-size: 15px; //15px
    line-height: 26px; //26px
	@include tablet {
        font-size: 16px; //12px
        line-height: 28px; //18px
	}
	@include desktop {
        font-size: 16px; //12px
        line-height: 28px; //18px
	}
}
@mixin p2-body-medium {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 16px; //13px
	line-height: 28px; //20px
	@include tablet {
        font-size: 13px; //13px
        line-height: 20px; //20px
	}
	@include desktop {
        font-size: 13px; //13px
        line-height: 20px; //20px
	}
}
@mixin p2-body-small {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 11px; //13px
	line-height: 16px; //20px
	@include tablet {
        font-size: 12px; //13px
        line-height: 18px; //20px
	}
	@include desktop {
        font-size: 12px; //13px
        line-height: 18px; //20px
	}
}

/*******************************************H5-Labels***********************/

@mixin h5-labels {
    font-size: 0.8rem; //12px
    line-height: 1.3rem; //20px 
    font-family: AvenirNextLTProRegular;
    color: $silver; //labels
    @include tablet {
        font-size: 0.8rem; //12px
        line-height: 1.3rem; // 20px
        font-family: AvenirNextLTProDemi;
    }
    @include desktop {
        font-size: 0.8rem; //12px
        line-height: 1.8rem; // 20px
    }
}

@mixin h5-labels-demi {
    @include h5-labels;
    font-family: AvenirNextLTProDemi;
}

@mixin h5-labels-bold {
    @include h5-labels;
    font-family: AvenirNextLTProBold;
}


//Breakpoints
@mixin tablet {
    @media (min-width: $breakpoint-tablet) {
        @content;
    }
}

@mixin desktop {
    @media (min-width: $breakpoint-desktop) {
        @content;
    }
}

//Text Styles- Desktop

@mixin d-header {
    font-family: $font-family;
    font-size: 1.875rem;
    line-height: 2.25rem;
    font-weight: 700;
	color: $text-black;
}

@mixin d-subheader {
	font-family: $font-family;
    font-size: 1.875rem;
    line-height: 2.25rem;
    font-weight: 500;
	color: $text-black;
}

@mixin d-subheader-grey {
	font-family: $font-family;
    font-size: 1.75rem;
    line-height: 2rem;
	color: $text-grey;
    font-weight: 500;
}

@mixin d-paragraph-bold {
	font-family: $font-family;
    font-size: 1.125rem;
    line-height: 1.25rem;
	color: $text-black;
    font-weight: 700;
}

@mixin d-paragraph-large {
	font-family: $font-family;
    font-size: 1.375rem;
    line-height: 1.4rem;
	color: $text-grey;
    font-weight: 400;
}

@mixin d-paragraph-medium {
	font-family: $font-family;
    font-size: 1.25rem;
    line-height: 1.4rem;
	color: $text-grey;
    font-weight: 400;
}

@mixin d-paragraph-small {
	font-family: $font-family;
    font-size: 1.4rem;
    line-height: 1rem;
	color: $text-grey;
    font-weight: 400;
}

//Button Styling
@mixin button-blue {
    background-color: $secondary-color-indigo;
    border-radius: 5px;
    color: white;
    padding: 0.875rem 1.813rem;
    appearance: none;
    border: none;
}

//Component Styling

@mixin component-styling {
    box-shadow: 0px 10px 25px rgba(19, 24, 44, 0.1);
    border-radius: 10px;
    padding: 3.75rem;
}
/***************************************STANDARD PADDING**************************************************/
@mixin standard-padding-right {
	padding-right: $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding-right: $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding-right: $mockup-column-outside-desktop; //80px right 80px left
	}
  }

  @mixin standard-padding-left {
	padding-left: $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding-left: $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding-left: $mockup-column-outside-desktop; //80px right 80px left
	}
  }
  
  @mixin standard-padding-horizontal {
	padding: 0 $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding: 0 $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding: 0 $mockup-column-outside-desktop; //80px right 80px left
	}
  }
  
  
  /**********************************WIDTH*100%*PADDING**********************************************/
  
  @mixin width-100-percent-minus-standard-padding-horizontal {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent} * 2));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent} * 2));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent} * 2));
	}
  }
  
  @mixin width-100-percent-minus-standard-padding-left {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent}));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent}));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent}));
	}
  }
  
  @mixin width-100-percent-minus-standard-padding-right {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent}));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent}));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent}));
	}
  }
  

  /*******************WIDTH MINUS MOCKUP COLUMN************/
  @mixin width-100-percent-minus-mockup-column-outside {
	width: calc(100% - #{$mockup-column-outside-mobile});
	@include tablet {
	  width: calc(100% - #{$mockup-column-outside-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-outside-desktop});
	}
  }
  @mixin width-100-percent-minus-mockup-column-outside-horizontal {
	width: calc(100% - (#{$mockup-column-outside-mobile} * 2));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet} *2));
	}
	@include desktop {
	  width: calc(100% - (#{mockup-column-outside-desktop} *2));
	}
  }
  @mixin width-100-percent-minus-mockup-column-small {
	width: calc(100% - #{mockup-column-small-mobile});
	@include tablet {
	  width: calc(100% -#{mockup-column-small-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-small-desktop});
	}
  }
  
  @mixin width-100-percent-minus-mockup-column-large {
	width: calc(100% - #{mockup-column-large-mobile});
	@include tablet {
	  width: calc(100% -#{mockup-column-large-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-large-desktop});
	}
  }
  



@mixin fancybutton {
  font-family: 'Source Sans Pro', sans-serif;
  font-weight: 900;
  padding: 1.25rem 2rem;
  font-size: 1rem;
  border-radius: 3.5rem / 100%;
  position: relative;
  min-width: 15rem;
  max-width: 90vw;
  overflow: hidden;
  border: 0;
  cursor: pointer;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  transition: all $time;
  
  & + & {
    margin-top: 1rem;
  }
  
  span {
    position: relative;
    z-index: 1;
  }
  
  &:before {
    content: "";
    background-color: #21D4FD;
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    transform: scale(4) translateX(-100%);
    transition: all $time * 1.5 ease-out;
  }
  
  &:hover,
  &:focus,
  &:active {    
    &:before {
      transform: scale(4) translate(37%);
    } 
  }
}


  /**********************************************DIVIDERS******************************************************/
  
  @mixin divider {
	background: $silver;
	width: 100%;
	height: 1px;
  }
  
  @mixin divider-form {
	background: $silver;
	width: 100%;
	height: 1px;
	margin-bottom: $increment2-px;
  }
  
  /**********************************************LOGOS******************************************************/
  
  @mixin logo-brainflix {
	img {
	  width: 120px; //120px
	  height: 24px; //24px
	  margin-left: 1rem;
	  @include tablet {
		margin-left: 2rem;
		width: 120px; //120px
		height: 24px; //24px
	  }
	  @include desktop {
		margin-left: 5rem;
	  }
	}
  }
  
  /**********************************************BUTTONS******************************************************/
  
  @mixin button {
	background-color: $brainflix-blue;
	@include h4-paragraphs-and-comments-white-demi;
	&:hover {
	  background: $brainflix-dark-blue-hover;
	}
	&:focus {
	  outline: none;
	}
	width: $increment28-px; //226px
	height: $increment6-px; 
	border-radius: $increment0-px; 
	border-color: $brainflix-blue;
	@include tablet {
	  padding: 0;
	  width: $increment20-px; //160px
	  height: $increment6-px; 
	}
  }
  
  /**********************************************MOCKUP COLUMNS****************************************************************/
  
  /*******************RIGHT*****************/
  
  @mixin mockup-column-outside-right {
	margin-right: $mockup-column-outside-mobile;
	@include tablet {
	  margin-right: $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-right {
	margin-right: $mockup-column-small-mobile;
	@include tablet {
	  margin-right: $mockup-column-small-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-right {
	margin-right: $mockup-column-large-mobile;
	@include tablet {
	  margin-right: $mockup-column-large-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-large-desktop;
	}
  }
  
  /*******************LEFT***************/
  
  @mixin mockup-column-outside-left {
	margin-left: $mockup-column-outside-mobile;
	@include tablet {
	  margin-left: $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-left {
	margin-left: $mockup-column-small-mobile;
	@include tablet {
	  margin-left: $mockup-column-small-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-left {
	margin-left: $mockup-column-large-mobile;
	@include tablet {
	  margin-left: $mockup-column-large-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-large-desktop;
	}
  }
  
  /*********************HORIZONTAL**************/
  
  @mixin mockup-column-outside-horizontal {
	margin: 0 $mockup-column-outside-mobile;
	@include tablet {
	  margin: 0 $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-horizontal {
	margin: 0 $mockup-column-small-mobile;
	@include tablet {
	  margin: 0 $mockup-column-small-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-horizontal {
	margin: 0 $mockup-column-large-mobile;
	@include tablet {
	  margin: 0 $mockup-column-large-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-large-desktop;
	}
  }
  
  /*********************************************WIDTH 100% MINUS INCREMENTS*********************************/
  
  @mixin width-100-percent-minus-increment0 {
	width: calc(100% - #{increment0-mobile});
	@include tablet {
	  width: calc(100% -#{increment0-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment0-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment1 {
	width: calc(100% - #{increment1-mobile});
	@include tablet {
	  width: calc(100% - #{increment1-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment1-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment2 {
	width: calc(100% - #{increment2-mobile});
	@include tablet {
	  width: calc(100% -#{increment2-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment2-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment3 {
	width: calc(100% - #{increment3-mobile});
	@include tablet {
	  width: calc(100% -#{increment3-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment3-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment4 {
	width: calc(100% - #{increment4-mobile});
	@include tablet {
	  width: calc(100% -#{increment4-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment4-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment5 {
	width: calc(100% - #{increment5-mobile});
	@include tablet {
	  width: calc(100% -#{increment5-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment5-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment6 {
	width: calc(100% - #{increment6-mobile});
	@include tablet {
	  width: calc(100% -#{increment6-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment6-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment7 {
	width: calc(100% - #{increment7-mobile});
	@include tablet {
	  width: calc(100% -#{increment7-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment7-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment8 {
	width: calc(100% - #{increment8-mobile});
	@include tablet {
	  width: calc(100% -#{increment8-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment8-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment9 {
	width: calc(100% - #{increment9-mobile});
	@include tablet {
	  width: calc(100% -#{increment9-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment9-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment10 {
	width: calc(100% - #{increment10-mobile});
	@include tablet {
	  width: calc(100% -#{increment10-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment10-desktop});
	}
  }
  

  
  /************************************%%%%%%%%%%%%%%%%%%%%%******************************************************
  /******%%%%%%%%%%%%%%%%%%%%%%%****CONTENTS LIST:*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****TOP MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *****%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%******RIGHT MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***BOTTOM MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***LEFT MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ***********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*VERTICAL MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ***********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*HORIZTONAL**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***WIDTH **********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%****HEIGHT**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****OTHER COMMON INCREMENTS*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
   ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****MOCKUP COLUMN PADDING*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********/
  
  /*******************************************************INCREMENT MARGIN TOP******************************************/
  
  @mixin increment-0-top {
	margin-top: $increment0-mobile;
	@include tablet {
	  margin-top: $increment0-tablet;
	}
	@include desktop {
	  margin-top: $increment0-desktop;
	}
  }
  
  @mixin increment-1-top {
	margin-top: $increment1-mobile;
	@include tablet {
	  margin-top: $increment1-tablet;
	}
	@include desktop {
	  margin-top: $increment1-desktop;
	}
  }
  
  @mixin increment-2-top {
	margin-top: $increment2-mobile;
	@include tablet {
	  margin-top: $increment2-tablet;
	}
	@include desktop {
	  margin-top: $increment2-desktop;
	}
  }
  
  @mixin increment-3-top {
	margin-top: $increment3-mobile;
	@include tablet {
	  margin-top: $increment3-tablet;
	}
	@include desktop {
	  margin-top: $increment3-desktop;
	}
  }
  
  @mixin increment-4-top {
	margin-top: $increment4-mobile;
	@include tablet {
	  margin-top: $increment4-tablet;
	}
	@include desktop {
	  margin-top: $increment4-desktop;
	}
  }
  
  @mixin increment-5-top {
	margin-top: $increment5-mobile;
	@include tablet {
	  margin-top: $increment5-tablet;
	}
	@include desktop {
	  margin-top: $increment5-desktop;
	}
  }
  
  @mixin increment-6-top {
	margin-top: $increment6-mobile;
	@include tablet {
	  margin-top: $increment6-tablet;
	}
	@include desktop {
	  margin-top: $increment6-desktop;
	}
  }
  
  @mixin increment-7-top {
	margin-top: $increment7-mobile;
	@include tablet {
	  margin-top: $increment7-tablet;
	}
	@include desktop {
	  margin-top: $increment7-desktop;
	}
  }
  
  @mixin increment-8-top {
	margin-top: $increment8-mobile;
	@include tablet {
	  margin-top: $increment8-tablet;
	}
	@include desktop {
	  margin-top: $increment8--desktop;
	}
  }
  
  @mixin increment-9-top {
	margin-top: $increment9-mobile;
	@include tablet {
	  margin-top: $increment9-tablet;
	}
	@include desktop {
	  margin-top: $increment9--desktop;
	}
  }
  
  @mixin increment-10-top {
	margin-top: $increment10-mobile;
	@include tablet {
	  margin-top: $increment10-tablet;
	}
	@include desktop {
	  margin-top: $increment10--desktop;
	}
  }
  
  /***********************************INCREMENT RIGHT*************************************************/
  
  @mixin increment-0-right {
	margin-right: $increment0-mobile;
	@include tablet {
	  margin-right: $increment0-tablet;
	}
	@include desktop {
	  margin-right: $increment0-desktop;
	}
  }
  
  @mixin increment-1-right {
	margin-right: $increment1-mobile;
	@include tablet {
	  margin-right: $increment1-tablet;
	}
	@include desktop {
	  margin-right: $increment1-desktop;
	}
  }
  
  @mixin increment-2-right {
	margin-right: $increment2-mobile;
	@include tablet {
	  margin-right: $increment2-tablet;
	}
	@include desktop {
	  margin-right: $increment2-desktop;
	}
  }
  
  @mixin increment-3-right {
	margin-right: $increment3-mobile;
	@include tablet {
	  margin-right: $increment3-tablet;
	}
	@include desktop {
	  margin-right: $increment3-desktop;
	}
  }
  
  @mixin increment-4-right {
	margin-right: $increment4-mobile;
	@include tablet {
	  margin-right: $increment4-tablet;
	}
	@include desktop {
	  margin-right: $increment4-desktop;
	}
  }
  
  @mixin increment-5-right {
	margin-right: $increment5-mobile;
	@include tablet {
	  margin-right: $increment5-tablet;
	}
	@include desktop {
	  margin-right: $increment5-desktop;
	}
  }
  
  @mixin increment-6-right {
	margin-right: $increment6-mobile;
	@include tablet {
	  margin-right: $increment6-tablet;
	}
	@include desktop {
	  margin-right: $increment6-desktop;
	}
  }
  
  @mixin increment-7-right {
	margin-right: $increment7-mobile;
	@include tablet {
	  margin-right: $increment7-tablet;
	}
	@include desktop {
	  margin-right: $increment7-desktop;
	}
  }
  
  @mixin increment-8-right {
	margin-right: $increment8-mobile;
	@include tablet {
	  margin-right: $increment8-tablet;
	}
	@include desktop {
	  margin-right: $increment8--desktop;
	}
  }
  
  @mixin increment-9-right {
	margin-right: $increment9-mobile;
	@include tablet {
	  margin-right: $increment9-tablet;
	}
	@include desktop {
	  margin-right: $increment9--desktop;
	}
  }
  
  @mixin increment-10-right {
	margin-right: $increment10-mobile;
	@include tablet {
	  margin-right: $increment10-tablet;
	}
	@include desktop {
	  margin-right: $increment10--desktop;
	}
  }
  
  /*******************************************************INCREMENT MARGIN BOTTOM******************************************/
  
  @mixin increment-0-bottom {
	margin-bottom: $increment0-mobile;
	@include tablet {
	  margin-bottom: $increment0-tablet;
	}
	@include desktop {
	  margin-bottom: $increment0-desktop;
	}
  }
  
  @mixin increment-1-bottom {
	margin-bottom: $increment1-mobile;
	@include tablet {
	  margin-bottom: $increment1-tablet;
	}
	@include desktop {
	  margin-bottom: $increment1-desktop;
	}
  }
  
  @mixin increment-2-bottom {
	margin-bottom: $increment2-mobile;
	@include tablet {
	  margin-bottom: $increment2-tablet;
	}
	@include desktop {
	  margin-bottom: $increment2-desktop;
	}
  }
  
  @mixin increment-3-bottom {
	margin-bottom: $increment3-mobile;
	@include tablet {
	  margin-bottom: $increment3-tablet;
	}
	@include desktop {
	  margin-bottom: $increment3-desktop;
	}
  }
  
  @mixin increment-4-bottom {
	margin-bottom: $increment4-mobile;
	@include tablet {
	  margin-bottom: $increment4-tablet;
	}
	@include desktop {
	  margin-bottom: $increment4-desktop;
	}
  }
  
  @mixin increment-5-bottom {
	margin-bottom: $increment5-mobile;
	@include tablet {
	  margin-bottom: $increment5-tablet;
	}
	@include desktop {
	  margin-bottom: $increment5-desktop;
	}
  }
  
  @mixin increment-6-bottom {
	margin-bottom: $increment6-mobile;
	@include tablet {
	  margin-bottom: $increment6-tablet;
	}
	@include desktop {
	  margin-bottom: $increment6-desktop;
	}
  }
  
  @mixin increment-7-bottom {
	margin-bottom: $increment7-mobile;
	@include tablet {
	  margin-bottom: $increment7-tablet;
	}
	@include desktop {
	  margin-bottom: $increment7-desktop;
	}
  }
  
  @mixin increment-8-bottom {
	margin-bottom: $increment8-mobile;
	@include tablet {
	  margin-bottom: $increment8-tablet;
	}
	@include desktop {
	  margin-bottom: $increment8--desktop;
	}
  }
  
  @mixin increment-9-bottom {
	margin-bottom: $increment9-mobile;
	@include tablet {
	  margin-bottom: $increment9-tablet;
	}
	@include desktop {
	  margin-bottom: $increment9--desktop;
	}
  }
  
  @mixin increment-10-bottom {
	margin-bottom: $increment10-mobile;
	@include tablet {
	  margin-bottom: $increment10-tablet;
	}
	@include desktop {
	  margin-bottom: $increment10--desktop;
	}
  }
  
  /*******************************************************INCREMENT LEFT******************************************/
  
  @mixin increment-0-left {
	margin-left: $increment0-mobile;
	@include tablet {
	  margin-left: $increment0-tablet;
	}
	@include desktop {
	  margin-left: $increment0-desktop;
	}
  }
  
  @mixin increment-1-left {
	margin-left: $increment1-mobile;
	@include tablet {
	  margin-left: $increment1-tablet;
	}
	@include desktop {
	  margin-left: $increment1-desktop;
	}
  }
  
  @mixin increment-2-left {
	margin-left: $increment2-mobile;
	@include tablet {
	  margin-left: $increment2-tablet;
	}
	@include desktop {
	  margin-left: $increment2-desktop;
	}
  }
  
  @mixin increment-3-left {
	margin-left: $increment3-mobile;
	@include tablet {
	  margin-left: $increment3-tablet;
	}
	@include desktop {
	  margin-left: $increment3-desktop;
	}
  }
  
  @mixin increment-4-left {
	margin-left: $increment4-mobile;
	@include tablet {
	  margin-left: $increment4-tablet;
	}
	@include desktop {
	  margin-left: $increment4-desktop;
	}
  }
  
  @mixin increment-5-left {
	margin-left: $increment5-mobile;
	@include tablet {
	  margin-left: $increment5-tablet;
	}
	@include desktop {
	  margin-left: $increment5-desktop;
	}
  }
  
  @mixin increment-6-left {
	margin-left: $increment6-mobile;
	@include tablet {
	  margin-left: $increment6-tablet;
	}
	@include desktop {
	  margin-left: $increment6-desktop;
	}
  }
  
  @mixin increment-7-left {
	margin-left: $increment7-mobile;
	@include tablet {
	  margin-left: $increment7-tablet;
	}
	@include desktop {
	  margin-left: $increment7-desktop;
	}
  }
  
  @mixin increment-8-left {
	margin-left: $increment8-mobile;
	@include tablet {
	  margin-left: $increment8-tablet;
	}
	@include desktop {
	  margin-left: $increment8--desktop;
	}
  }
  
  @mixin increment-9-left {
	margin-left: $increment9-mobile;
	@include tablet {
	  margin-left: $increment9-tablet;
	}
	@include desktop {
	  margin-left: $increment9--desktop;
	}
  }
  
  @mixin increment-10-left {
	margin-left: $increment10-mobile;
	@include tablet {
	  margin-left: $increment10-tablet;
	}
	@include desktop {
	  margin-left: $increment10--desktop;
	}
  }
  
  /************************************VERTICAL MARGIN INCREMENTS******************************************************/
  
  @mixin increment-0-vertical {
	margin: $increment0-mobile 0;
	@include tablet {
	  margin: $increment0-tablet 0;
	}
	@include deskvertical {
	  margin: $increment0-desktop 0;
	}
  }
  
  @mixin increment-1-vertical {
	margin: $increment1-mobile 0;
	@include tablet {
	  margin: $increment1-tablet 0;
	}
	@include desktop {
	  margin: $increment1-desktop 0;
	}
  }
  
  @mixin increment-2-vertical {
	margin: $increment2-mobile 0;
	@include tablet {
	  margin: $increment2-tablet 0;
	}
	@include desktop {
	  margin: $increment2-desktop 0;
	}
  }
  
  @mixin increment-3-vertical {
	margin: $increment3-mobile 0;
	@include tablet {
	  margin: $increment3-tablet 0;
	}
	@include desktop {
	  margin: $increment3-desktop 0;
	}
  }
  
  @mixin increment-4-vertical {
	margin: $increment4-mobile 0;
	@include tablet {
	  margin: $increment4-tablet 0;
	}
	@include desktop {
	  margin: $increment4-desktop 0;
	}
  }
  
  @mixin increment-5-vertical {
	margin: $increment5-mobile 0;
	@include tablet {
	  margin: $increment5-tablet 0;
	}
	@include desktop {
	  margin: $increment5-desktop 0;
	}
  }
  
  @mixin increment-6-vertical {
	margin: $increment6-mobile 0;
	@include tablet {
	  margin: $increment6-tablet 0;
	}
	@include desktop {
	  margin: $increment6-desktop 0;
	}
  }
  
  @mixin increment-7-vertical {
	margin: $increment7-mobile 0;
	@include tablet {
	  margin: $increment7-tablet 0;
	}
	@include desktop {
	  margin: $increment7-desktop 0;
	}
  }
  
  @mixin increment-8-vertical {
	margin: $increment8-mobile 0;
	@include tablet {
	  margin: $increment8-tablet 0;
	}
	@include desktop {
	  margin: $increment8--desktop 0;
	}
  }
  
  @mixin increment-9-vertical {
	margin: $increment9-mobile 0;
	@include tablet {
	  margin: $increment9-tablet 0;
	}
	@include desktop {
	  margin: $increment9--desktop 0;
	}
  }
  
  @mixin increment-10-vertical {
	margin: $increment10-mobile 0;
	@include tablet {
	  margin: $increment10-tablet 0;
	}
	@include desktop {
	  margin: $increment10--desktop 0;
	}
  }
  
  /************************************HORIZONTAL MARGIN INCREMENTS******************************************************/
  
  @mixin increment-0-horizontal {
	margin: 0 $increment0-mobile;
	@include tablet {
	  margin: 0 $increment0-tablet;
	}
	@include deskhorizontal {
	  margin: 0 $increment0-desktop;
	}
  }
  
  @mixin increment-1-horizontal {
	margin: 0 $increment1-mobile;
	@include tablet {
	  margin: 0 $increment1-tablet;
	}
	@include desktop {
	  margin: 0 $increment1-desktop;
	}
  }
  
  @mixin increment-2-horizontal {
	margin: 0 $increment2-mobile;
	@include tablet {
	  margin: 0 $increment2-tablet;
	}
	@include desktop {
	  margin: 0 $increment2-desktop;
	}
  }
  
  @mixin increment-3-horizontal {
	margin: 0 $increment3-mobile;
	@include tablet {
	  margin: 0 $increment3-tablet;
	}
	@include desktop {
	  margin: 0 $increment3-desktop;
	}
  }
  
  @mixin increment-4-horizontal {
	margin: 0 $increment4-mobile;
	@include tablet {
	  margin: 0 $increment4-tablet;
	}
	@include desktop {
	  margin: 0 $increment4-desktop;
	}
  }
  
  @mixin increment-5-horizontal {
	margin: 0 $increment5-mobile;
	@include tablet {
	  margin: 0 $increment5-tablet;
	}
	@include desktop {
	  margin: 0 $increment5-desktop;
	}
  }
  
  @mixin increment-6-horizontal {
	margin: 0 $increment6-mobile;
	@include tablet {
	  margin: 0 $increment6-tablet;
	}
	@include desktop {
	  margin: 0 $increment6-desktop;
	}
  }
  
  @mixin increment-7-horizontal {
	margin: 0 $increment7-mobile;
	@include tablet {
	  margin: 0 $increment7-tablet;
	}
	@include desktop {
	  margin: 0 $increment7-desktop;
	}
  }
  
  @mixin increment-8-horizontal {
	margin: 0 $increment8-mobile;
	@include tablet {
	  margin: 0 $increment8-tablet;
	}
	@include desktop {
	  margin: 0 $increment8--desktop;
	}
  }
  
  @mixin increment-9-horizontal {
	margin: 0 $increment9-mobile;
	@include tablet {
	  margin: 0 $increment9-tablet;
	}
	@include desktop {
	  margin: 0 $increment9--desktop;
	}
  }
  
  @mixin increment-10-horizontal {
	margin: 0 $increment10-mobile;
	@include tablet {
	  margin: 0 $increment10-tablet;
	}
	@include desktop {
	  margin: 0 $increment10--desktop;
	}
  }
    /***************************************************INCREMENT WIDTH******************************************/
  
	@mixin increment-0-width {
		width: $increment0-mobile;
		@include tablet {
		  width: $increment0-tablet;
		}
		@include deskwidth {
		  width: $increment0-desktop;
		}
	  }
	  
	  @mixin increment-1-width {
		width: $increment1-mobile;
		@include tablet {
		  width: $increment1-tablet;
		}
		@include desktop {
		  width: $increment1-desktop;
		}
	  }
	  
	  @mixin increment-2-width {
		width: $increment2-mobile;
		@include tablet {
		  width: $increment2-tablet;
		}
		@include desktop {
		  width: $increment2-desktop;
		}
	  }
	  
	  @mixin increment-3-width {
		width: $increment3-mobile;
		@include tablet {
		  width: $increment3-tablet;
		}
		@include desktop {
		  width: $increment3-desktop;
		}
	  }
	  
	  @mixin increment-4-width {
		width: $increment4-mobile;
		@include tablet {
		  width: $increment4-tablet;
		}
		@include desktop {
		  width: $increment4-desktop;
		}
	  }
	  
	  @mixin increment-5-width {
		width: $increment5-mobile;
		@include tablet {
		  width: $increment5-tablet;
		}
		@include desktop {
		  width: $increment5-desktop;
		}
	  }
	  
	  @mixin increment-6-width {
		width: $increment6-mobile;
		@include tablet {
		  width: $increment6-tablet;
		}
		@include desktop {
		  width: $increment6-desktop;
		}
	  }
	  
	  @mixin increment-7-width {
		width: $increment7-mobile;
		@include tablet {
		  width: $increment7-tablet;
		}
		@include desktop {
		  width: $increment7-desktop;
		}
	  }
	  
	  @mixin increment-8-width {
		width: $increment8-mobile;
		@include tablet {
		  width: $increment8-tablet;
		}
		@include desktop {
		  width: $increment8--desktop;
		}
	  }
	  
	  @mixin increment-9-width {
		width: $increment9-mobile;
		@include tablet {
		  width: $increment9-tablet;
		}
		@include desktop {
		  width: $increment9--desktop;
		}
	  }
	  
	  @mixin increment-10-width {
		width: $increment10-mobile;
		@include tablet {
		  width: $increment10-tablet;
		}
		@include desktop {
		  width: $increment10--desktop;
		}
	  }
  
  /***************************************************INCREMENT HEIGHT******************************************/
  
  @mixin increment-0-height {
	height: $increment0-mobile;
	@include tablet {
	  height: $increment0-tablet;
	}
	@include deskheight {
	  height: $increment0-desktop;
	}
  }
  
  @mixin increment-1-height {
	height: $increment1-mobile;
	@include tablet {
	  height: $increment1-tablet;
	}
	@include desktop {
	  height: $increment1-desktop;
	}
  }
  
  @mixin increment-2-height {
	height: $increment2-mobile;
	@include tablet {
	  height: $increment2-tablet;
	}
	@include desktop {
	  height: $increment2-desktop;
	}
  }
  
  @mixin increment-3-height {
	height: $increment3-mobile;
	@include tablet {
	  height: $increment3-tablet;
	}
	@include desktop {
	  height: $increment3-desktop;
	}
  }
  
  @mixin increment-4-height {
	height: $increment4-mobile;
	@include tablet {
	  height: $increment4-tablet;
	}
	@include desktop {
	  height: $increment4-desktop;
	}
  }
  
  @mixin increment-5-height {
	height: $increment5-mobile;
	@include tablet {
	  height: $increment5-tablet;
	}
	@include desktop {
	  height: $increment5-desktop;
	}
  }
  
  @mixin increment-6-height {
	height: $increment6-mobile;
	@include tablet {
	  height: $increment6-tablet;
	}
	@include desktop {
	  height: $increment6-desktop;
	}
  }
  
  @mixin increment-7-height {
	height: $increment7-mobile;
	@include tablet {
	  height: $increment7-tablet;
	}
	@include desktop {
	  height: $increment7-desktop;
	}
  }
  
  @mixin increment-8-height {
	height: $increment8-mobile;
	@include tablet {
	  height: $increment8-tablet;
	}
	@include desktop {
	  height: $increment8--desktop;
	}
  }
  
  @mixin increment-9-height {
	height: $increment9-mobile;
	@include tablet {
	  height: $increment9-tablet;
	}
	@include desktop {
	  height: $increment9--desktop;
	}
  }
  
  @mixin increment-10-height {
	height: $increment10-mobile;
	@include tablet {
	  height: $increment10-tablet;
	}
	@include desktop {
	  height: $increment10--desktop;
	}
  }
  
  /***************************************************OTHER COMMON INCREMENTS********************************/
  
  /*******TOP****/
  
  @mixin increment20px-top {
	margin-top: $increment20px-mobile;
	@include tablet {
	  margin-top: $increment20px-tablet;
	}
	@include desktop {
	  margin-top: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-top {
	margin-top: $increment54px-mobile;
	@include tablet {
	  margin-top: $increment54px-tablet;
	}
	@include desktop {
	  margin-top: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-top {
	margin-top: $increment68px-mobile;
	@include tablet {
	  margin-top: $increment68px-tablet;
	}
	@include desktop {
	  margin-top: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-top {
	margin-top: $increment100px-mobile;
	@include tablet {
	  margin-top: $increment100px-tablet;
	}
	@include desktop {
	  margin-top: $increment100px-desktop;
	}
  }
  
  /***************RIGHT********************/
  
  @mixin increment20px-right {
	margin-right: $increment20px-mobile;
	@include tablet {
	  margin-right: $increment20px-tablet;
	}
	@include desktop {
	  margin-right: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-right {
	margin-right: $increment54px-mobile;
	@include tablet {
	  margin-right: $increment54px-tablet;
	}
	@include desktop {
	  margin-right: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-right {
	margin-right: $increment68px-mobile;
	@include tablet {
	  margin-right: $increment68px-tablet;
	}
	@include desktop {
	  margin-right: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-right {
	margin-right: $increment100px-mobile;
	@include tablet {
	  margin-right: $increment100px-tablet;
	}
	@include desktop {
	  margin-right: $increment100px-desktop;
	}
  }
  
  /****************BOTTOM***************/
  
  @mixin increment20px-bottom {
	margin-bottom: $increment20px-mobile;
	@include tablet {
	  margin-bottom: $increment20px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-bottom {
	margin-bottom: $increment54px-mobile;
	@include tablet {
	  margin-bottom: $increment54px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-bottom {
	margin-bottom: $increment68px-mobile;
	@include tablet {
	  margin-bottom: $increment68px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-bottom {
	margin-bottom: $increment100px-mobile;
	@include tablet {
	  margin-bottom: $increment100px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment100px-desktop;
	}
  }
  
  /*****************LEFT****************/
  
  @mixin increment20px-left {
	margin-left: $increment20px-mobile;
	@include tablet {
	  margin-left: $increment20px-tablet;
	}
	@include desktop {
	  margin-left: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-left {
	margin-left: $increment54px-mobile;
	@include tablet {
	  margin-left: $increment54px-tablet;
	}
	@include desktop {
	  margin-left: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-left {
	margin-left: $increment68px-mobile;
	@include tablet {
	  margin-left: $increment68px-tablet;
	}
	@include desktop {
	  margin-left: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-left {
	margin-left: $increment100px-mobile;
	@include tablet {
	  margin-left: $increment100px-tablet;
	}
	@include desktop {
	  margin-left: $increment100px-desktop;
	}
  }
  
  /************VERTICAL***********/
  
  @mixin increment20px-vertical {
	margin: $increment20px-mobile 0;
	@include tablet {
	  margin: $increment20px-tablet 0;
	}
	@include desktop {
	  margin: $increment20px-desktop 0;
	}
  }
  
  @mixin increment54px-vertical {
	margin: $increment54px-mobile 0;
	@include tablet {
	  margin: $increment54px-tablet 0;
	}
	@include desktop {
	  margin: $increment54px-desktop 0;
	}
  }
  
  @mixin increment68px-vertical {
	margin: $increment68px-mobile 0;
	@include tablet {
	  margin: $increment68px-tablet 0;
	}
	@include desktop {
	  margin: $increment68px-desktop 0;
	}
  }
  
  @mixin increment100px-vertical {
	margin: $increment100px-mobile 0;
	@include tablet {
	  margin: $increment100px-tablet 0;
	}
	@include desktop {
	  margin: $increment100px-desktop 0;
	}
  }
  
  /**********HORIZONTAL*********/
  
  @mixin increment20px-horizontal {
	margin: 0 $increment20px-mobile;
	@include tablet {
	  margin: 0 $increment20px-tablet;
	}
	@include desktop {
	  margin: 0 $increment20px-desktop;
	}
  }
  
  @mixin increment54px-horizontal {
	margin: 0 $increment54px-mobile;
	@include tablet {
	  margin: 0 $increment54px-tablet;
	}
	@include desktop {
	  margin: 0 $increment54px-desktop;
	}
  }
  
  @mixin increment68px-horizontal {
	margin: 0 $increment68px-mobile;
	@include tablet {
	  margin: 0 $increment68px-tablet;
	}
	@include desktop {
	  margin: 0 $increment68px-desktop;
	}
  }
  
  @mixin increment100px-horizontal {
	margin: 0 $increment100px-mobile;
	@include tablet {
	  margin: 0 $increment100px-tablet;
	}
	@include desktop {
	  margin: 0 $increment100px-desktop;
	}
  }
  
 /***********WIDTH*********/
  
  @mixin increment20px-width {
	width: $increment20px-mobile;
	@include tablet {
	  width: $increment20px-tablet;
	}
	@include desktop {
	  width: $increment20px-desktop;
	}
  }

 @mixin increment54px-width {
	width: $increment54px-mobile;
	@include tablet {
	  width: $increment54px-tablet;
	}
	@include desktop {
	  width: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-width {
	width: $increment68px-mobile;
	@include tablet {
	  width: $increment68px-tablet;
	}
	@include desktop {
	  width: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-width {
	width: $increment100px-mobile;
	@include tablet {
	  width: $increment100px-tablet;
	}
	@include desktop {
	  width: $increment100px-desktop;
	}
  }

  /***********HEIGHT*********/
  @mixin increment20px-height {
	height: $increment20px-mobile;
	@include tablet {
	  height: $increment20px-tablet;
	}
	@include desktop {
	  height: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-height {
	height: $increment54px-mobile;
	@include tablet {
	  height: $increment54px-tablet;
	}
	@include desktop {
	  height: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-height {
	height: $increment68px-mobile;
	@include tablet {
	  height: $increment68px-tablet;
	}
	@include desktop {
	  height: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-height {
	height: $increment100px-mobile;
	@include tablet {
	  height: $increment100px-tablet;
	}
	@include desktop {
	  height: $increment100px-desktop;
	}
  }
  
  /**********************************************MOCKUP COLUMNS PADDING****************************************************************/
  
  @mixin mockup-column-padding-outside-right {
	padding-right: $mockup-column-outside-mobile;
	@include tablet {
	  padding-right: $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-right {
	padding-right: $mockup-column-small-mobile;
	@include tablet {
	  padding-right: $mockup-column-small-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-right {
	padding-right: $mockup-column-large-mobile;
	@include tablet {
	  padding-right: $mockup-column-large-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-large-desktop;
	}
  }
  
  /***LEFT*******/
  
  @mixin mockup-column-padding-outside-left {
	padding-left: $mockup-column-outside-mobile;
	@include tablet {
	  padding-left: $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-left {
	padding-left: $mockup-column-small-mobile;
	@include tablet {
	  padding-left: $mockup-column-small-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-left {
	padding-left: $mockup-column-large-mobile;
	@include tablet {
	  padding-left: $mockup-column-large-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-large-desktop;
	}
  }
  
  /***HORIZONTAL*******/
  
  @mixin mockup-column-padding-outside-horizontal {
	padding: 0 $mockup-column-outside-mobile;
	@include tablet {
	  padding: 0 $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-horizontal {
	padding: 0 $mockup-column-small-mobile;
	@include tablet {
	  padding: 0 $mockup-column-small-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-horizontal {
	padding: 0 $mockup-column-large-mobile;
	@include tablet {
	  padding: 0 $mockup-column-large-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-large-desktop;
	}
  }
  /*************************************TEXT STYLES************************************/
@use "./colors" as *;
@use "./breakpoints" as *;
@use "./variables" as *;
/*****************************************H1-Main-heading***************************/

@mixin h1-main-header {
    font-family: AvenirNextLTProBold;
    font-size: 1.5rem; //24px 
    line-height: 2.0rem; //32px
    color: $white;
    @include tablet {
        font-size: 2.3rem; //36px 2.25rem 
        line-height: 2.8rem; //44px
    }
    @include desktop {
        font-size: 2.3rem; //36px 2.25rem 
        line-height: 2.8rem; //44px
    }
}

@mixin h1-main-header-regular {
    @include h1-main-header;
    font-family: AvenirNextLTProRegular;
}

@mixin h1-main-header-black {
    @include h1-main-header;
    color: $black;
}

@mixin h1-desktop-size-that-should-be-available-on-mobile {
    color: $white;
    font-family: AvenirNextLTProBold;
    font-size: 8.0vw;
    line-height: 10.8vw;
}


/*********************H2 - Header subheader for mobile*********************/

@mixin h2-header {
    font-size: 0.9rem; //14px 
    line-height: 1.4rem; //22px 
    font-family: AvenirNextLTProDemi;
    color: $black;
    @include tablet {
        font-size: 1.5rem; //24px 1.5rem 
        line-height: 2.0; //32px 2rem
        font-family: AvenirNextLTProBold;
    }
    @include desktop {
        font-size: 1.5rem; //24px 1.5rem 
        line-height: 2.0rem; //32px 2rem 
    }
}

@mixin h2-header-white {
    @include h2-header;
    color: $white;
}

@mixin h2-header-responsive-white {
    @include h2-header-responsive;
    color: $white;
}

@mixin h2-desktop-size-that-should-be-available-on-mobile {
    color: $white;
    font-size: 1.5rem; //24px 1.5rem 
    line-height: 2.0; //32px 2rem
    font-family: AvenirNextLTProBold;
}


/*******************************H3 - Subheader for Tablet and Desktop************/

@mixin h3-subheader {
    font-size: 1.0rem; //16px 
    line-height: 1.5rem; // 24px
    font-family: AvenirNextLTProDemi;
    color: $black;
    @include tablet {
        font-size: 1.0rem; //16px 
        line-height: 1.5rem; // 24px
    }
    @include desktop {
        font-size: 1.0rem; //16px 
        line-height: 1.5rem; // 24px
    }
}

@mixin h3-subheader-white {
    @include h3-subheader;
    color: $white;
}

@mixin h3-subheader-bold {
    @include h3-subheader;
    font-family: AvenirNextLTProBold;
}

@mixin h3-subheader-white-bold {
    @include h3-subheader;
    font-family: AvenirNextLTProBold;
    color: $white;
}


/********************H4 Paragraph & Comments******************************/

@mixin h4-paragraphs-and-comments {
    font-family: AvenirNextLTProRegular;
    color: $black;
    font-size: 0.8rem; // 13px
    line-height: 1.1rem; //18px
    @include tablet {
        font-size: 0.9rem; // 14px
        line-height: 1.4rem; //22px
    }
    @include desktop {
        font-size: 0.9rem; // 14px
        line-height: 1.4rem; // 22px 
    }
}

@mixin h4-paragraphs-and-comments-bold {
    @include h4-paragraphs-and-comments;
    font-family: AvenirNextLTProBold;
}

@mixin h4-paragraphs-and-comments-white {
    @include h4-paragraphs-and-comments;
    color: $white;
}

@mixin h4-paragraphs-and-comments-white-bold {
    @include h4-paragraphs-and-comments-bold;
    color: $white;
}

@mixin h4-paragraphs-and-comments-white-demi {
    @include h4-paragraphs-and-comments-white;
    font-family: AvenirNextLTProDemi;
    color: $white;
}

$time: 330ms;
$btns: ( 
alpha: ( gstart: #21D4FD,
gend: #B721FF,
tstart: #B721FF,
tend: #21D4FD),
beta: ( gstart: #08AEEA,
gend: #2AF598,
tstart: #2AF598,
tend: #08AEEA),
gamma: ( gstart: #FEE140,
gend: #FA709A,
tstart: #FA709A,
tend: #FEE140),
delta: ( gstart: #3EECAC,
gend: #EE74E1,
tstart: #EE74E1,
tend: #3EECAC));
.btn {
	outline: none;;
	margin-right: 30px;
	margin-top: 30px;
	font-family: 'Source Sans Pro', sans-serif;
	font-weight: 900;
	padding: 1.25rem 2rem;
	font-size: 1rem;
	border-radius: 3.5rem / 100%;
	position: relative;
	min-width: 15rem;
	max-width: 90vw;
	overflow: hidden;
	border: 0;
	cursor: pointer;
	text-transform: uppercase;
	letter-spacing: 0.05em;
	transition: all $time;
	outline: none;;
	&+& {
		outline: none;;
		margin-top: 1rem;
	}
	span {
		outline: none;;
		position: relative;
		z-index: 1;
	}
	&:before {
		outline: none;;
		content: "";
		background-color: #21D4FD;
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		transform: scale(4) translateX(-100%);
		transition: all $time * 1.5 ease-out;
		outline: none;;
	}
	&:hover,
	&:focus,
	&:active {
		&:before {
			outline: none;;
			transform: scale(4) translate(37%);
		}
	}
}

@each $bname,
$bcolors in $btns {
	.btn--#{$bname} {
		outline: none;;
		background-color: map-get($bcolors, gstart);
		color: map-get($bcolors, tstart);
		&:before {
			background-color: map-get($bcolors, gend);
			background-image: linear-gradient(to right, map-get($bcolors, gend) 30%, map-get($bcolors, gstart) 100%);
			position: absolute;
		}
		&:hover,
		&:focus,
		&:active {
			outline: none;;
			color: map-get($bcolors, tend);
		}
	}
}

.btn-form {
	outline: none;;
	outline: none;;
	margin-right: 30px;
	margin-top: 30px;
	font-family: 'Source Sans Pro', sans-serif;
	font-weight: 900;
	padding: 1.25rem 2rem;
	font-size: 1rem;
	border-radius: 3.5rem / 100%;
	position: relative;
	min-width: 15rem;
	max-width: 90vw;
	overflow: hidden;
	border: 0;
	cursor: pointer;
	text-transform: uppercase;
	letter-spacing: 0.05em;
	transition: all $time;
	 ;
	&+& {
		margin-top: 1rem;
		 ;
	}
	span {
		position: relative;
		z-index: 1;
		 ;
	}
	&:before {
		content: "";
		background-color: #21D4FD;
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		transform: scale(4) translateX(-100%);
		transition: all $time * 1.5 ease-out;
		 ;
	}
	&:hover,
	&:focus,
	&:active {
		&:before {
			 
			//transform: scale(4) translate(37%);
		  //  color: map-get($bcolors, tstart);
		  background-color:#B721FF;;
		}
	}
}

@each $bname,
$bcolors in $btns {
	.btn-form--#{$bname} {
		background-color: map-get($bcolors, gstart);
		color: map-get($bcolors, tstart);
		&:before {
			background-color: map-get($bcolors, gend);
			background-image: linear-gradient(to right, map-get($bcolors, gend) 30%, map-get($bcolors, gstart) 100%);
			position: absolute;
		}
		&:hover,
		&:focus,
		&:active {
		//	color: map-get($bcolors, tstart);
		color: #B721FF;
		}
	}
}
/********************************h4-lists-and-contacts-information*************/

@mixin h4-list-contact-information {
    color: $black;
    font-family: AvenirNextLTProRegular;
    font-size: 0.8rem; //13px 
    line-height: 1.1rem; //18px 
    @include tablet {
        font-size: 0.9rem; //14px 
        line-height: 1.3rem; //20px
    }
    @include desktop {
        font-size: 0.9rem; //14px 
        line-height: 1.3rem; //20px 
    }
}

@mixin h4-list-contact-information-bold {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProBold;
}

@mixin h4-list-contact-information-demi {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProDemi;
}

@mixin h4-list-contact-information-white {
    @include h4-list-contact-information;
    color: $white;
}

@mixin h4-list-contact-information-white-demi {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProDemi;
    color: $white;
}

@mixin h4-list-contact-information-white-bold {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProBold;
    color: $white;
}


//********************************P-Paragraph****************************/

@mixin p-paragraph {
    font-family: AvenirNextLTProRegular;
    color: $black;
    font-size: 0.8rem; //13px
    line-height: 1.1rem; //18px
    @include tablet {
        font-size: 0.9rem; //13px 
        line-height: 1.3rem; //18px 
        display: inline-block;
    }
    @include desktop {
        font-size: 0.9rem; //13px 
        line-height: 1.3rem; //18px
        display: inline-block;
    }
}

@mixin p-paragraph-bold {
    @include p-paragraph;
    font-family: AvenirNextLTProBold;
}

@mixin p-paragraph-white {
    @include p-paragraph;
    color: white;
}

@mixin p-paragraph-white-bold {
    @include p-paragraph-bold;
    color: white;
}

@mixin h1-page-header {
	font-family: TitilliumWebSemiBold600;
	color: $black;
	font-size: 28px; //28px
	line-height: 36px; //36px
	@include tablet {
        font-size: 32px; //32px
        line-height: 40px; //40px
	}
	@include desktop {
        font-size: 32px; //32px
        line-height: 40px; //40px
	}
}

@mixin h2-subheader {
	font-family: TitilliumWebSemiBold600;
	color: $black;
	font-size: 20px; //20px
	line-height: 28px; //28px
	@include tablet {
        font-size: 24px; //24px
        line-height: 32px; //28px
	}
	@include desktop {
        font-size: 24px; //24px
        line-height: 32px; //28px
	}
}
@mixin h3-labels-links-buttons {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 13px; //13px
	line-height: 20px; //20px
	@include tablet {
        font-size: 14px; //14px
        line-height: 22px; //22px
	}
	@include desktop {
        font-size: 14px; //14px
        line-height: 22px; //22px
	}
}
@mixin h4-labels-table-header {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 11px; //11px
	line-height: 16px; //16px
	@include tablet {
        font-size: 12px; //12px
        line-height: 18px; //18px
	}
	@include desktop {
        font-size: 12px; //12px
        line-height: 18px; //18px
	}
}
@mixin p1-body-large {
	font-family: TitilliumWebRegular400;
	color: $black;
    font-size: 15px; //15px
    line-height: 26px; //26px
	@include tablet {
        font-size: 16px; //12px
        line-height: 28px; //18px
	}
	@include desktop {
        font-size: 16px; //12px
        line-height: 28px; //18px
	}
}
@mixin p2-body-medium {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 16px; //13px
	line-height: 28px; //20px
	@include tablet {
        font-size: 13px; //13px
        line-height: 20px; //20px
	}
	@include desktop {
        font-size: 13px; //13px
        line-height: 20px; //20px
	}
}
@mixin p2-body-small {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 11px; //13px
	line-height: 16px; //20px
	@include tablet {
        font-size: 12px; //13px
        line-height: 18px; //20px
	}
	@include desktop {
        font-size: 12px; //13px
        line-height: 18px; //20px
	}
}

/*******************************************H5-Labels***********************/

@mixin h5-labels {
    font-size: 0.8rem; //12px
    line-height: 1.3rem; //20px 
    font-family: AvenirNextLTProRegular;
    color: $silver; //labels
    @include tablet {
        font-size: 0.8rem; //12px
        line-height: 1.3rem; // 20px
        font-family: AvenirNextLTProDemi;
    }
    @include desktop {
        font-size: 0.8rem; //12px
        line-height: 1.8rem; // 20px
    }
}

@mixin h5-labels-demi {
    @include h5-labels;
    font-family: AvenirNextLTProDemi;
}

@mixin h5-labels-bold {
    @include h5-labels;
    font-family: AvenirNextLTProBold;
}


//Breakpoints
@mixin tablet {
    @media (min-width: $breakpoint-tablet) {
        @content;
    }
}

@mixin desktop {
    @media (min-width: $breakpoint-desktop) {
        @content;
    }
}

//Text Styles- Desktop

@mixin d-header {
    font-family: $font-family;
    font-size: 1.875rem;
    line-height: 2.25rem;
    font-weight: 700;
	color: $text-black;
}

@mixin d-subheader {
	font-family: $font-family;
    font-size: 1.875rem;
    line-height: 2.25rem;
    font-weight: 500;
	color: $text-black;
}

@mixin d-subheader-grey {
	font-family: $font-family;
    font-size: 1.75rem;
    line-height: 2rem;
	color: $text-grey;
    font-weight: 500;
}

@mixin d-paragraph-bold {
	font-family: $font-family;
    font-size: 1.125rem;
    line-height: 1.25rem;
	color: $text-black;
    font-weight: 700;
}

@mixin d-paragraph-large {
	font-family: $font-family;
    font-size: 1.375rem;
    line-height: 1.4rem;
	color: $text-grey;
    font-weight: 400;
}

@mixin d-paragraph-medium {
	font-family: $font-family;
    font-size: 1.25rem;
    line-height: 1.4rem;
	color: $text-grey;
    font-weight: 400;
}

@mixin d-paragraph-small {
	font-family: $font-family;
    font-size: 1.4rem;
    line-height: 1rem;
	color: $text-grey;
    font-weight: 400;
}

//Button Styling
@mixin button-blue {
    background-color: $secondary-color-indigo;
    border-radius: 5px;
    color: white;
    padding: 0.875rem 1.813rem;
    appearance: none;
    border: none;
}

//Component Styling

@mixin component-styling {
    box-shadow: 0px 10px 25px rgba(19, 24, 44, 0.1);
    border-radius: 10px;
    padding: 3.75rem;
}
/***************************************STANDARD PADDING**************************************************/
@mixin standard-padding-right {
	padding-right: $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding-right: $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding-right: $mockup-column-outside-desktop; //80px right 80px left
	}
  }

  @mixin standard-padding-left {
	padding-left: $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding-left: $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding-left: $mockup-column-outside-desktop; //80px right 80px left
	}
  }
  
  @mixin standard-padding-horizontal {
	padding: 0 $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding: 0 $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding: 0 $mockup-column-outside-desktop; //80px right 80px left
	}
  }
  
  
  /**********************************WIDTH*100%*PADDING**********************************************/
  
  @mixin width-100-percent-minus-standard-padding-horizontal {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent} * 2));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent} * 2));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent} * 2));
	}
  }
  
  @mixin width-100-percent-minus-standard-padding-left {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent}));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent}));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent}));
	}
  }
  
  @mixin width-100-percent-minus-standard-padding-right {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent}));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent}));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent}));
	}
  }
  

  /*******************WIDTH MINUS MOCKUP COLUMN************/
  @mixin width-100-percent-minus-mockup-column-outside {
	width: calc(100% - #{$mockup-column-outside-mobile});
	@include tablet {
	  width: calc(100% - #{$mockup-column-outside-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-outside-desktop});
	}
  }
  @mixin width-100-percent-minus-mockup-column-outside-horizontal {
	width: calc(100% - (#{$mockup-column-outside-mobile} * 2));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet} *2));
	}
	@include desktop {
	  width: calc(100% - (#{mockup-column-outside-desktop} *2));
	}
  }
  @mixin width-100-percent-minus-mockup-column-small {
	width: calc(100% - #{mockup-column-small-mobile});
	@include tablet {
	  width: calc(100% -#{mockup-column-small-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-small-desktop});
	}
  }
  
  @mixin width-100-percent-minus-mockup-column-large {
	width: calc(100% - #{mockup-column-large-mobile});
	@include tablet {
	  width: calc(100% -#{mockup-column-large-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-large-desktop});
	}
  }
  



@mixin fancybutton {
  font-family: 'Source Sans Pro', sans-serif;
  font-weight: 900;
  padding: 1.25rem 2rem;
  font-size: 1rem;
  border-radius: 3.5rem / 100%;
  position: relative;
  min-width: 15rem;
  max-width: 90vw;
  overflow: hidden;
  border: 0;
  cursor: pointer;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  transition: all $time;
  
  & + & {
    margin-top: 1rem;
  }
  
  span {
    position: relative;
    z-index: 1;
  }
  
  &:before {
    content: "";
    background-color: #21D4FD;
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    transform: scale(4) translateX(-100%);
    transition: all $time * 1.5 ease-out;
  }
  
  &:hover,
  &:focus,
  &:active {    
    &:before {
      transform: scale(4) translate(37%);
    } 
  }
}


  /**********************************************DIVIDERS******************************************************/
  
  @mixin divider {
	background: $silver;
	width: 100%;
	height: 1px;
  }
  
  @mixin divider-form {
	background: $silver;
	width: 100%;
	height: 1px;
	margin-bottom: $increment2-px;
  }
  
  /**********************************************LOGOS******************************************************/
  
  @mixin logo-brainflix {
	img {
	  width: 120px; //120px
	  height: 24px; //24px
	  margin-left: 1rem;
	  @include tablet {
		margin-left: 2rem;
		width: 120px; //120px
		height: 24px; //24px
	  }
	  @include desktop {
		margin-left: 5rem;
	  }
	}
  }
  
  /**********************************************BUTTONS******************************************************/
  
  @mixin button {
	background-color: $brainflix-blue;
	@include h4-paragraphs-and-comments-white-demi;
	&:hover {
	  background: $brainflix-dark-blue-hover;
	}
	&:focus {
	  outline: none;
	}
	width: $increment28-px; //226px
	height: $increment6-px; 
	border-radius: $increment0-px; 
	border-color: $brainflix-blue;
	@include tablet {
	  padding: 0;
	  width: $increment20-px; //160px
	  height: $increment6-px; 
	}
  }
  
  /**********************************************MOCKUP COLUMNS****************************************************************/
  
  /*******************RIGHT*****************/
  
  @mixin mockup-column-outside-right {
	margin-right: $mockup-column-outside-mobile;
	@include tablet {
	  margin-right: $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-right {
	margin-right: $mockup-column-small-mobile;
	@include tablet {
	  margin-right: $mockup-column-small-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-right {
	margin-right: $mockup-column-large-mobile;
	@include tablet {
	  margin-right: $mockup-column-large-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-large-desktop;
	}
  }
  
  /*******************LEFT***************/
  
  @mixin mockup-column-outside-left {
	margin-left: $mockup-column-outside-mobile;
	@include tablet {
	  margin-left: $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-left {
	margin-left: $mockup-column-small-mobile;
	@include tablet {
	  margin-left: $mockup-column-small-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-left {
	margin-left: $mockup-column-large-mobile;
	@include tablet {
	  margin-left: $mockup-column-large-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-large-desktop;
	}
  }
  
  /*********************HORIZONTAL**************/
  
  @mixin mockup-column-outside-horizontal {
	margin: 0 $mockup-column-outside-mobile;
	@include tablet {
	  margin: 0 $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-horizontal {
	margin: 0 $mockup-column-small-mobile;
	@include tablet {
	  margin: 0 $mockup-column-small-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-horizontal {
	margin: 0 $mockup-column-large-mobile;
	@include tablet {
	  margin: 0 $mockup-column-large-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-large-desktop;
	}
  }
  
  /*********************************************WIDTH 100% MINUS INCREMENTS*********************************/
  
  @mixin width-100-percent-minus-increment0 {
	width: calc(100% - #{increment0-mobile});
	@include tablet {
	  width: calc(100% -#{increment0-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment0-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment1 {
	width: calc(100% - #{increment1-mobile});
	@include tablet {
	  width: calc(100% - #{increment1-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment1-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment2 {
	width: calc(100% - #{increment2-mobile});
	@include tablet {
	  width: calc(100% -#{increment2-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment2-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment3 {
	width: calc(100% - #{increment3-mobile});
	@include tablet {
	  width: calc(100% -#{increment3-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment3-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment4 {
	width: calc(100% - #{increment4-mobile});
	@include tablet {
	  width: calc(100% -#{increment4-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment4-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment5 {
	width: calc(100% - #{increment5-mobile});
	@include tablet {
	  width: calc(100% -#{increment5-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment5-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment6 {
	width: calc(100% - #{increment6-mobile});
	@include tablet {
	  width: calc(100% -#{increment6-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment6-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment7 {
	width: calc(100% - #{increment7-mobile});
	@include tablet {
	  width: calc(100% -#{increment7-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment7-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment8 {
	width: calc(100% - #{increment8-mobile});
	@include tablet {
	  width: calc(100% -#{increment8-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment8-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment9 {
	width: calc(100% - #{increment9-mobile});
	@include tablet {
	  width: calc(100% -#{increment9-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment9-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment10 {
	width: calc(100% - #{increment10-mobile});
	@include tablet {
	  width: calc(100% -#{increment10-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment10-desktop});
	}
  }
  

  
  /************************************%%%%%%%%%%%%%%%%%%%%%******************************************************
  /******%%%%%%%%%%%%%%%%%%%%%%%****CONTENTS LIST:*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****TOP MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *****%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%******RIGHT MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***BOTTOM MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***LEFT MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ***********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*VERTICAL MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ***********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*HORIZTONAL**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***WIDTH **********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%****HEIGHT**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****OTHER COMMON INCREMENTS*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
   ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****MOCKUP COLUMN PADDING*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********/
  
  /*******************************************************INCREMENT MARGIN TOP******************************************/
  
  @mixin increment-0-top {
	margin-top: $increment0-mobile;
	@include tablet {
	  margin-top: $increment0-tablet;
	}
	@include desktop {
	  margin-top: $increment0-desktop;
	}
  }
  
  @mixin increment-1-top {
	margin-top: $increment1-mobile;
	@include tablet {
	  margin-top: $increment1-tablet;
	}
	@include desktop {
	  margin-top: $increment1-desktop;
	}
  }
  
  @mixin increment-2-top {
	margin-top: $increment2-mobile;
	@include tablet {
	  margin-top: $increment2-tablet;
	}
	@include desktop {
	  margin-top: $increment2-desktop;
	}
  }
  
  @mixin increment-3-top {
	margin-top: $increment3-mobile;
	@include tablet {
	  margin-top: $increment3-tablet;
	}
	@include desktop {
	  margin-top: $increment3-desktop;
	}
  }
  
  @mixin increment-4-top {
	margin-top: $increment4-mobile;
	@include tablet {
	  margin-top: $increment4-tablet;
	}
	@include desktop {
	  margin-top: $increment4-desktop;
	}
  }
  
  @mixin increment-5-top {
	margin-top: $increment5-mobile;
	@include tablet {
	  margin-top: $increment5-tablet;
	}
	@include desktop {
	  margin-top: $increment5-desktop;
	}
  }
  
  @mixin increment-6-top {
	margin-top: $increment6-mobile;
	@include tablet {
	  margin-top: $increment6-tablet;
	}
	@include desktop {
	  margin-top: $increment6-desktop;
	}
  }
  
  @mixin increment-7-top {
	margin-top: $increment7-mobile;
	@include tablet {
	  margin-top: $increment7-tablet;
	}
	@include desktop {
	  margin-top: $increment7-desktop;
	}
  }
  
  @mixin increment-8-top {
	margin-top: $increment8-mobile;
	@include tablet {
	  margin-top: $increment8-tablet;
	}
	@include desktop {
	  margin-top: $increment8--desktop;
	}
  }
  
  @mixin increment-9-top {
	margin-top: $increment9-mobile;
	@include tablet {
	  margin-top: $increment9-tablet;
	}
	@include desktop {
	  margin-top: $increment9--desktop;
	}
  }
  
  @mixin increment-10-top {
	margin-top: $increment10-mobile;
	@include tablet {
	  margin-top: $increment10-tablet;
	}
	@include desktop {
	  margin-top: $increment10--desktop;
	}
  }
  
  /***********************************INCREMENT RIGHT*************************************************/
  
  @mixin increment-0-right {
	margin-right: $increment0-mobile;
	@include tablet {
	  margin-right: $increment0-tablet;
	}
	@include desktop {
	  margin-right: $increment0-desktop;
	}
  }
  
  @mixin increment-1-right {
	margin-right: $increment1-mobile;
	@include tablet {
	  margin-right: $increment1-tablet;
	}
	@include desktop {
	  margin-right: $increment1-desktop;
	}
  }
  
  @mixin increment-2-right {
	margin-right: $increment2-mobile;
	@include tablet {
	  margin-right: $increment2-tablet;
	}
	@include desktop {
	  margin-right: $increment2-desktop;
	}
  }
  
  @mixin increment-3-right {
	margin-right: $increment3-mobile;
	@include tablet {
	  margin-right: $increment3-tablet;
	}
	@include desktop {
	  margin-right: $increment3-desktop;
	}
  }
  
  @mixin increment-4-right {
	margin-right: $increment4-mobile;
	@include tablet {
	  margin-right: $increment4-tablet;
	}
	@include desktop {
	  margin-right: $increment4-desktop;
	}
  }
  
  @mixin increment-5-right {
	margin-right: $increment5-mobile;
	@include tablet {
	  margin-right: $increment5-tablet;
	}
	@include desktop {
	  margin-right: $increment5-desktop;
	}
  }
  
  @mixin increment-6-right {
	margin-right: $increment6-mobile;
	@include tablet {
	  margin-right: $increment6-tablet;
	}
	@include desktop {
	  margin-right: $increment6-desktop;
	}
  }
  
  @mixin increment-7-right {
	margin-right: $increment7-mobile;
	@include tablet {
	  margin-right: $increment7-tablet;
	}
	@include desktop {
	  margin-right: $increment7-desktop;
	}
  }
  
  @mixin increment-8-right {
	margin-right: $increment8-mobile;
	@include tablet {
	  margin-right: $increment8-tablet;
	}
	@include desktop {
	  margin-right: $increment8--desktop;
	}
  }
  
  @mixin increment-9-right {
	margin-right: $increment9-mobile;
	@include tablet {
	  margin-right: $increment9-tablet;
	}
	@include desktop {
	  margin-right: $increment9--desktop;
	}
  }
  
  @mixin increment-10-right {
	margin-right: $increment10-mobile;
	@include tablet {
	  margin-right: $increment10-tablet;
	}
	@include desktop {
	  margin-right: $increment10--desktop;
	}
  }
  
  /*******************************************************INCREMENT MARGIN BOTTOM******************************************/
  
  @mixin increment-0-bottom {
	margin-bottom: $increment0-mobile;
	@include tablet {
	  margin-bottom: $increment0-tablet;
	}
	@include desktop {
	  margin-bottom: $increment0-desktop;
	}
  }
  
  @mixin increment-1-bottom {
	margin-bottom: $increment1-mobile;
	@include tablet {
	  margin-bottom: $increment1-tablet;
	}
	@include desktop {
	  margin-bottom: $increment1-desktop;
	}
  }
  
  @mixin increment-2-bottom {
	margin-bottom: $increment2-mobile;
	@include tablet {
	  margin-bottom: $increment2-tablet;
	}
	@include desktop {
	  margin-bottom: $increment2-desktop;
	}
  }
  
  @mixin increment-3-bottom {
	margin-bottom: $increment3-mobile;
	@include tablet {
	  margin-bottom: $increment3-tablet;
	}
	@include desktop {
	  margin-bottom: $increment3-desktop;
	}
  }
  
  @mixin increment-4-bottom {
	margin-bottom: $increment4-mobile;
	@include tablet {
	  margin-bottom: $increment4-tablet;
	}
	@include desktop {
	  margin-bottom: $increment4-desktop;
	}
  }
  
  @mixin increment-5-bottom {
	margin-bottom: $increment5-mobile;
	@include tablet {
	  margin-bottom: $increment5-tablet;
	}
	@include desktop {
	  margin-bottom: $increment5-desktop;
	}
  }
  
  @mixin increment-6-bottom {
	margin-bottom: $increment6-mobile;
	@include tablet {
	  margin-bottom: $increment6-tablet;
	}
	@include desktop {
	  margin-bottom: $increment6-desktop;
	}
  }
  
  @mixin increment-7-bottom {
	margin-bottom: $increment7-mobile;
	@include tablet {
	  margin-bottom: $increment7-tablet;
	}
	@include desktop {
	  margin-bottom: $increment7-desktop;
	}
  }
  
  @mixin increment-8-bottom {
	margin-bottom: $increment8-mobile;
	@include tablet {
	  margin-bottom: $increment8-tablet;
	}
	@include desktop {
	  margin-bottom: $increment8--desktop;
	}
  }
  
  @mixin increment-9-bottom {
	margin-bottom: $increment9-mobile;
	@include tablet {
	  margin-bottom: $increment9-tablet;
	}
	@include desktop {
	  margin-bottom: $increment9--desktop;
	}
  }
  
  @mixin increment-10-bottom {
	margin-bottom: $increment10-mobile;
	@include tablet {
	  margin-bottom: $increment10-tablet;
	}
	@include desktop {
	  margin-bottom: $increment10--desktop;
	}
  }
  
  /*******************************************************INCREMENT LEFT******************************************/
  
  @mixin increment-0-left {
	margin-left: $increment0-mobile;
	@include tablet {
	  margin-left: $increment0-tablet;
	}
	@include desktop {
	  margin-left: $increment0-desktop;
	}
  }
  
  @mixin increment-1-left {
	margin-left: $increment1-mobile;
	@include tablet {
	  margin-left: $increment1-tablet;
	}
	@include desktop {
	  margin-left: $increment1-desktop;
	}
  }
  
  @mixin increment-2-left {
	margin-left: $increment2-mobile;
	@include tablet {
	  margin-left: $increment2-tablet;
	}
	@include desktop {
	  margin-left: $increment2-desktop;
	}
  }
  
  @mixin increment-3-left {
	margin-left: $increment3-mobile;
	@include tablet {
	  margin-left: $increment3-tablet;
	}
	@include desktop {
	  margin-left: $increment3-desktop;
	}
  }
  
  @mixin increment-4-left {
	margin-left: $increment4-mobile;
	@include tablet {
	  margin-left: $increment4-tablet;
	}
	@include desktop {
	  margin-left: $increment4-desktop;
	}
  }
  
  @mixin increment-5-left {
	margin-left: $increment5-mobile;
	@include tablet {
	  margin-left: $increment5-tablet;
	}
	@include desktop {
	  margin-left: $increment5-desktop;
	}
  }
  
  @mixin increment-6-left {
	margin-left: $increment6-mobile;
	@include tablet {
	  margin-left: $increment6-tablet;
	}
	@include desktop {
	  margin-left: $increment6-desktop;
	}
  }
  
  @mixin increment-7-left {
	margin-left: $increment7-mobile;
	@include tablet {
	  margin-left: $increment7-tablet;
	}
	@include desktop {
	  margin-left: $increment7-desktop;
	}
  }
  
  @mixin increment-8-left {
	margin-left: $increment8-mobile;
	@include tablet {
	  margin-left: $increment8-tablet;
	}
	@include desktop {
	  margin-left: $increment8--desktop;
	}
  }
  
  @mixin increment-9-left {
	margin-left: $increment9-mobile;
	@include tablet {
	  margin-left: $increment9-tablet;
	}
	@include desktop {
	  margin-left: $increment9--desktop;
	}
  }
  
  @mixin increment-10-left {
	margin-left: $increment10-mobile;
	@include tablet {
	  margin-left: $increment10-tablet;
	}
	@include desktop {
	  margin-left: $increment10--desktop;
	}
  }
  
  /************************************VERTICAL MARGIN INCREMENTS******************************************************/
  
  @mixin increment-0-vertical {
	margin: $increment0-mobile 0;
	@include tablet {
	  margin: $increment0-tablet 0;
	}
	@include deskvertical {
	  margin: $increment0-desktop 0;
	}
  }
  
  @mixin increment-1-vertical {
	margin: $increment1-mobile 0;
	@include tablet {
	  margin: $increment1-tablet 0;
	}
	@include desktop {
	  margin: $increment1-desktop 0;
	}
  }
  
  @mixin increment-2-vertical {
	margin: $increment2-mobile 0;
	@include tablet {
	  margin: $increment2-tablet 0;
	}
	@include desktop {
	  margin: $increment2-desktop 0;
	}
  }
  
  @mixin increment-3-vertical {
	margin: $increment3-mobile 0;
	@include tablet {
	  margin: $increment3-tablet 0;
	}
	@include desktop {
	  margin: $increment3-desktop 0;
	}
  }
  
  @mixin increment-4-vertical {
	margin: $increment4-mobile 0;
	@include tablet {
	  margin: $increment4-tablet 0;
	}
	@include desktop {
	  margin: $increment4-desktop 0;
	}
  }
  
  @mixin increment-5-vertical {
	margin: $increment5-mobile 0;
	@include tablet {
	  margin: $increment5-tablet 0;
	}
	@include desktop {
	  margin: $increment5-desktop 0;
	}
  }
  
  @mixin increment-6-vertical {
	margin: $increment6-mobile 0;
	@include tablet {
	  margin: $increment6-tablet 0;
	}
	@include desktop {
	  margin: $increment6-desktop 0;
	}
  }
  
  @mixin increment-7-vertical {
	margin: $increment7-mobile 0;
	@include tablet {
	  margin: $increment7-tablet 0;
	}
	@include desktop {
	  margin: $increment7-desktop 0;
	}
  }
  
  @mixin increment-8-vertical {
	margin: $increment8-mobile 0;
	@include tablet {
	  margin: $increment8-tablet 0;
	}
	@include desktop {
	  margin: $increment8--desktop 0;
	}
  }
  
  @mixin increment-9-vertical {
	margin: $increment9-mobile 0;
	@include tablet {
	  margin: $increment9-tablet 0;
	}
	@include desktop {
	  margin: $increment9--desktop 0;
	}
  }
  
  @mixin increment-10-vertical {
	margin: $increment10-mobile 0;
	@include tablet {
	  margin: $increment10-tablet 0;
	}
	@include desktop {
	  margin: $increment10--desktop 0;
	}
  }
  
  /************************************HORIZONTAL MARGIN INCREMENTS******************************************************/
  
  @mixin increment-0-horizontal {
	margin: 0 $increment0-mobile;
	@include tablet {
	  margin: 0 $increment0-tablet;
	}
	@include deskhorizontal {
	  margin: 0 $increment0-desktop;
	}
  }
  
  @mixin increment-1-horizontal {
	margin: 0 $increment1-mobile;
	@include tablet {
	  margin: 0 $increment1-tablet;
	}
	@include desktop {
	  margin: 0 $increment1-desktop;
	}
  }
  
  @mixin increment-2-horizontal {
	margin: 0 $increment2-mobile;
	@include tablet {
	  margin: 0 $increment2-tablet;
	}
	@include desktop {
	  margin: 0 $increment2-desktop;
	}
  }
  
  @mixin increment-3-horizontal {
	margin: 0 $increment3-mobile;
	@include tablet {
	  margin: 0 $increment3-tablet;
	}
	@include desktop {
	  margin: 0 $increment3-desktop;
	}
  }
  
  @mixin increment-4-horizontal {
	margin: 0 $increment4-mobile;
	@include tablet {
	  margin: 0 $increment4-tablet;
	}
	@include desktop {
	  margin: 0 $increment4-desktop;
	}
  }
  
  @mixin increment-5-horizontal {
	margin: 0 $increment5-mobile;
	@include tablet {
	  margin: 0 $increment5-tablet;
	}
	@include desktop {
	  margin: 0 $increment5-desktop;
	}
  }
  
  @mixin increment-6-horizontal {
	margin: 0 $increment6-mobile;
	@include tablet {
	  margin: 0 $increment6-tablet;
	}
	@include desktop {
	  margin: 0 $increment6-desktop;
	}
  }
  
  @mixin increment-7-horizontal {
	margin: 0 $increment7-mobile;
	@include tablet {
	  margin: 0 $increment7-tablet;
	}
	@include desktop {
	  margin: 0 $increment7-desktop;
	}
  }
  
  @mixin increment-8-horizontal {
	margin: 0 $increment8-mobile;
	@include tablet {
	  margin: 0 $increment8-tablet;
	}
	@include desktop {
	  margin: 0 $increment8--desktop;
	}
  }
  
  @mixin increment-9-horizontal {
	margin: 0 $increment9-mobile;
	@include tablet {
	  margin: 0 $increment9-tablet;
	}
	@include desktop {
	  margin: 0 $increment9--desktop;
	}
  }
  
  @mixin increment-10-horizontal {
	margin: 0 $increment10-mobile;
	@include tablet {
	  margin: 0 $increment10-tablet;
	}
	@include desktop {
	  margin: 0 $increment10--desktop;
	}
  }
    /***************************************************INCREMENT WIDTH******************************************/
  
	@mixin increment-0-width {
		width: $increment0-mobile;
		@include tablet {
		  width: $increment0-tablet;
		}
		@include deskwidth {
		  width: $increment0-desktop;
		}
	  }
	  
	  @mixin increment-1-width {
		width: $increment1-mobile;
		@include tablet {
		  width: $increment1-tablet;
		}
		@include desktop {
		  width: $increment1-desktop;
		}
	  }
	  
	  @mixin increment-2-width {
		width: $increment2-mobile;
		@include tablet {
		  width: $increment2-tablet;
		}
		@include desktop {
		  width: $increment2-desktop;
		}
	  }
	  
	  @mixin increment-3-width {
		width: $increment3-mobile;
		@include tablet {
		  width: $increment3-tablet;
		}
		@include desktop {
		  width: $increment3-desktop;
		}
	  }
	  
	  @mixin increment-4-width {
		width: $increment4-mobile;
		@include tablet {
		  width: $increment4-tablet;
		}
		@include desktop {
		  width: $increment4-desktop;
		}
	  }
	  
	  @mixin increment-5-width {
		width: $increment5-mobile;
		@include tablet {
		  width: $increment5-tablet;
		}
		@include desktop {
		  width: $increment5-desktop;
		}
	  }
	  
	  @mixin increment-6-width {
		width: $increment6-mobile;
		@include tablet {
		  width: $increment6-tablet;
		}
		@include desktop {
		  width: $increment6-desktop;
		}
	  }
	  
	  @mixin increment-7-width {
		width: $increment7-mobile;
		@include tablet {
		  width: $increment7-tablet;
		}
		@include desktop {
		  width: $increment7-desktop;
		}
	  }
	  
	  @mixin increment-8-width {
		width: $increment8-mobile;
		@include tablet {
		  width: $increment8-tablet;
		}
		@include desktop {
		  width: $increment8--desktop;
		}
	  }
	  
	  @mixin increment-9-width {
		width: $increment9-mobile;
		@include tablet {
		  width: $increment9-tablet;
		}
		@include desktop {
		  width: $increment9--desktop;
		}
	  }
	  
	  @mixin increment-10-width {
		width: $increment10-mobile;
		@include tablet {
		  width: $increment10-tablet;
		}
		@include desktop {
		  width: $increment10--desktop;
		}
	  }
  
  /***************************************************INCREMENT HEIGHT******************************************/
  
  @mixin increment-0-height {
	height: $increment0-mobile;
	@include tablet {
	  height: $increment0-tablet;
	}
	@include deskheight {
	  height: $increment0-desktop;
	}
  }
  
  @mixin increment-1-height {
	height: $increment1-mobile;
	@include tablet {
	  height: $increment1-tablet;
	}
	@include desktop {
	  height: $increment1-desktop;
	}
  }
  
  @mixin increment-2-height {
	height: $increment2-mobile;
	@include tablet {
	  height: $increment2-tablet;
	}
	@include desktop {
	  height: $increment2-desktop;
	}
  }
  
  @mixin increment-3-height {
	height: $increment3-mobile;
	@include tablet {
	  height: $increment3-tablet;
	}
	@include desktop {
	  height: $increment3-desktop;
	}
  }
  
  @mixin increment-4-height {
	height: $increment4-mobile;
	@include tablet {
	  height: $increment4-tablet;
	}
	@include desktop {
	  height: $increment4-desktop;
	}
  }
  
  @mixin increment-5-height {
	height: $increment5-mobile;
	@include tablet {
	  height: $increment5-tablet;
	}
	@include desktop {
	  height: $increment5-desktop;
	}
  }
  
  @mixin increment-6-height {
	height: $increment6-mobile;
	@include tablet {
	  height: $increment6-tablet;
	}
	@include desktop {
	  height: $increment6-desktop;
	}
  }
  
  @mixin increment-7-height {
	height: $increment7-mobile;
	@include tablet {
	  height: $increment7-tablet;
	}
	@include desktop {
	  height: $increment7-desktop;
	}
  }
  
  @mixin increment-8-height {
	height: $increment8-mobile;
	@include tablet {
	  height: $increment8-tablet;
	}
	@include desktop {
	  height: $increment8--desktop;
	}
  }
  
  @mixin increment-9-height {
	height: $increment9-mobile;
	@include tablet {
	  height: $increment9-tablet;
	}
	@include desktop {
	  height: $increment9--desktop;
	}
  }
  
  @mixin increment-10-height {
	height: $increment10-mobile;
	@include tablet {
	  height: $increment10-tablet;
	}
	@include desktop {
	  height: $increment10--desktop;
	}
  }
  
  /***************************************************OTHER COMMON INCREMENTS********************************/
  
  /*******TOP****/
  
  @mixin increment20px-top {
	margin-top: $increment20px-mobile;
	@include tablet {
	  margin-top: $increment20px-tablet;
	}
	@include desktop {
	  margin-top: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-top {
	margin-top: $increment54px-mobile;
	@include tablet {
	  margin-top: $increment54px-tablet;
	}
	@include desktop {
	  margin-top: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-top {
	margin-top: $increment68px-mobile;
	@include tablet {
	  margin-top: $increment68px-tablet;
	}
	@include desktop {
	  margin-top: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-top {
	margin-top: $increment100px-mobile;
	@include tablet {
	  margin-top: $increment100px-tablet;
	}
	@include desktop {
	  margin-top: $increment100px-desktop;
	}
  }
  
  /***************RIGHT********************/
  
  @mixin increment20px-right {
	margin-right: $increment20px-mobile;
	@include tablet {
	  margin-right: $increment20px-tablet;
	}
	@include desktop {
	  margin-right: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-right {
	margin-right: $increment54px-mobile;
	@include tablet {
	  margin-right: $increment54px-tablet;
	}
	@include desktop {
	  margin-right: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-right {
	margin-right: $increment68px-mobile;
	@include tablet {
	  margin-right: $increment68px-tablet;
	}
	@include desktop {
	  margin-right: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-right {
	margin-right: $increment100px-mobile;
	@include tablet {
	  margin-right: $increment100px-tablet;
	}
	@include desktop {
	  margin-right: $increment100px-desktop;
	}
  }
  
  /****************BOTTOM***************/
  
  @mixin increment20px-bottom {
	margin-bottom: $increment20px-mobile;
	@include tablet {
	  margin-bottom: $increment20px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-bottom {
	margin-bottom: $increment54px-mobile;
	@include tablet {
	  margin-bottom: $increment54px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-bottom {
	margin-bottom: $increment68px-mobile;
	@include tablet {
	  margin-bottom: $increment68px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-bottom {
	margin-bottom: $increment100px-mobile;
	@include tablet {
	  margin-bottom: $increment100px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment100px-desktop;
	}
  }
  
  /*****************LEFT****************/
  
  @mixin increment20px-left {
	margin-left: $increment20px-mobile;
	@include tablet {
	  margin-left: $increment20px-tablet;
	}
	@include desktop {
	  margin-left: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-left {
	margin-left: $increment54px-mobile;
	@include tablet {
	  margin-left: $increment54px-tablet;
	}
	@include desktop {
	  margin-left: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-left {
	margin-left: $increment68px-mobile;
	@include tablet {
	  margin-left: $increment68px-tablet;
	}
	@include desktop {
	  margin-left: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-left {
	margin-left: $increment100px-mobile;
	@include tablet {
	  margin-left: $increment100px-tablet;
	}
	@include desktop {
	  margin-left: $increment100px-desktop;
	}
  }
  
  /************VERTICAL***********/
  
  @mixin increment20px-vertical {
	margin: $increment20px-mobile 0;
	@include tablet {
	  margin: $increment20px-tablet 0;
	}
	@include desktop {
	  margin: $increment20px-desktop 0;
	}
  }
  
  @mixin increment54px-vertical {
	margin: $increment54px-mobile 0;
	@include tablet {
	  margin: $increment54px-tablet 0;
	}
	@include desktop {
	  margin: $increment54px-desktop 0;
	}
  }
  
  @mixin increment68px-vertical {
	margin: $increment68px-mobile 0;
	@include tablet {
	  margin: $increment68px-tablet 0;
	}
	@include desktop {
	  margin: $increment68px-desktop 0;
	}
  }
  
  @mixin increment100px-vertical {
	margin: $increment100px-mobile 0;
	@include tablet {
	  margin: $increment100px-tablet 0;
	}
	@include desktop {
	  margin: $increment100px-desktop 0;
	}
  }
  
  /**********HORIZONTAL*********/
  
  @mixin increment20px-horizontal {
	margin: 0 $increment20px-mobile;
	@include tablet {
	  margin: 0 $increment20px-tablet;
	}
	@include desktop {
	  margin: 0 $increment20px-desktop;
	}
  }
  
  @mixin increment54px-horizontal {
	margin: 0 $increment54px-mobile;
	@include tablet {
	  margin: 0 $increment54px-tablet;
	}
	@include desktop {
	  margin: 0 $increment54px-desktop;
	}
  }
  
  @mixin increment68px-horizontal {
	margin: 0 $increment68px-mobile;
	@include tablet {
	  margin: 0 $increment68px-tablet;
	}
	@include desktop {
	  margin: 0 $increment68px-desktop;
	}
  }
  
  @mixin increment100px-horizontal {
	margin: 0 $increment100px-mobile;
	@include tablet {
	  margin: 0 $increment100px-tablet;
	}
	@include desktop {
	  margin: 0 $increment100px-desktop;
	}
  }
  
 /***********WIDTH*********/
  
  @mixin increment20px-width {
	width: $increment20px-mobile;
	@include tablet {
	  width: $increment20px-tablet;
	}
	@include desktop {
	  width: $increment20px-desktop;
	}
  }

 @mixin increment54px-width {
	width: $increment54px-mobile;
	@include tablet {
	  width: $increment54px-tablet;
	}
	@include desktop {
	  width: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-width {
	width: $increment68px-mobile;
	@include tablet {
	  width: $increment68px-tablet;
	}
	@include desktop {
	  width: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-width {
	width: $increment100px-mobile;
	@include tablet {
	  width: $increment100px-tablet;
	}
	@include desktop {
	  width: $increment100px-desktop;
	}
  }

  /***********HEIGHT*********/
  @mixin increment20px-height {
	height: $increment20px-mobile;
	@include tablet {
	  height: $increment20px-tablet;
	}
	@include desktop {
	  height: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-height {
	height: $increment54px-mobile;
	@include tablet {
	  height: $increment54px-tablet;
	}
	@include desktop {
	  height: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-height {
	height: $increment68px-mobile;
	@include tablet {
	  height: $increment68px-tablet;
	}
	@include desktop {
	  height: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-height {
	height: $increment100px-mobile;
	@include tablet {
	  height: $increment100px-tablet;
	}
	@include desktop {
	  height: $increment100px-desktop;
	}
  }
  
  /**********************************************MOCKUP COLUMNS PADDING****************************************************************/
  
  @mixin mockup-column-padding-outside-right {
	padding-right: $mockup-column-outside-mobile;
	@include tablet {
	  padding-right: $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-right {
	padding-right: $mockup-column-small-mobile;
	@include tablet {
	  padding-right: $mockup-column-small-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-right {
	padding-right: $mockup-column-large-mobile;
	@include tablet {
	  padding-right: $mockup-column-large-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-large-desktop;
	}
  }
  
  /***LEFT*******/
  
  @mixin mockup-column-padding-outside-left {
	padding-left: $mockup-column-outside-mobile;
	@include tablet {
	  padding-left: $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-left {
	padding-left: $mockup-column-small-mobile;
	@include tablet {
	  padding-left: $mockup-column-small-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-left {
	padding-left: $mockup-column-large-mobile;
	@include tablet {
	  padding-left: $mockup-column-large-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-large-desktop;
	}
  }
  
  /***HORIZONTAL*******/
  
  @mixin mockup-column-padding-outside-horizontal {
	padding: 0 $mockup-column-outside-mobile;
	@include tablet {
	  padding: 0 $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-horizontal {
	padding: 0 $mockup-column-small-mobile;
	@include tablet {
	  padding: 0 $mockup-column-small-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-horizontal {
	padding: 0 $mockup-column-large-mobile;
	@include tablet {
	  padding: 0 $mockup-column-large-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-large-desktop;
	}
  }
  /*************************************TEXT STYLES************************************/
@use "./colors" as *;
@use "./breakpoints" as *;
@use "./variables" as *;
/*****************************************H1-Main-heading***************************/

@mixin h1-main-header {
    font-family: AvenirNextLTProBold;
    font-size: 1.5rem; //24px 
    line-height: 2.0rem; //32px
    color: $white;
    @include tablet {
        font-size: 2.3rem; //36px 2.25rem 
        line-height: 2.8rem; //44px
    }
    @include desktop {
        font-size: 2.3rem; //36px 2.25rem 
        line-height: 2.8rem; //44px
    }
}

@mixin h1-main-header-regular {
    @include h1-main-header;
    font-family: AvenirNextLTProRegular;
}

@mixin h1-main-header-black {
    @include h1-main-header;
    color: $black;
}

@mixin h1-desktop-size-that-should-be-available-on-mobile {
    color: $white;
    font-family: AvenirNextLTProBold;
    font-size: 8.0vw;
    line-height: 10.8vw;
}


/*********************H2 - Header subheader for mobile*********************/

@mixin h2-header {
    font-size: 0.9rem; //14px 
    line-height: 1.4rem; //22px 
    font-family: AvenirNextLTProDemi;
    color: $black;
    @include tablet {
        font-size: 1.5rem; //24px 1.5rem 
        line-height: 2.0; //32px 2rem
        font-family: AvenirNextLTProBold;
    }
    @include desktop {
        font-size: 1.5rem; //24px 1.5rem 
        line-height: 2.0rem; //32px 2rem 
    }
}

@mixin h2-header-white {
    @include h2-header;
    color: $white;
}

@mixin h2-header-responsive-white {
    @include h2-header-responsive;
    color: $white;
}

@mixin h2-desktop-size-that-should-be-available-on-mobile {
    color: $white;
    font-size: 1.5rem; //24px 1.5rem 
    line-height: 2.0; //32px 2rem
    font-family: AvenirNextLTProBold;
}


/*******************************H3 - Subheader for Tablet and Desktop************/

@mixin h3-subheader {
    font-size: 1.0rem; //16px 
    line-height: 1.5rem; // 24px
    font-family: AvenirNextLTProDemi;
    color: $black;
    @include tablet {
        font-size: 1.0rem; //16px 
        line-height: 1.5rem; // 24px
    }
    @include desktop {
        font-size: 1.0rem; //16px 
        line-height: 1.5rem; // 24px
    }
}

@mixin h3-subheader-white {
    @include h3-subheader;
    color: $white;
}

@mixin h3-subheader-bold {
    @include h3-subheader;
    font-family: AvenirNextLTProBold;
}

@mixin h3-subheader-white-bold {
    @include h3-subheader;
    font-family: AvenirNextLTProBold;
    color: $white;
}


/********************H4 Paragraph & Comments******************************/

@mixin h4-paragraphs-and-comments {
    font-family: AvenirNextLTProRegular;
    color: $black;
    font-size: 0.8rem; // 13px
    line-height: 1.1rem; //18px
    @include tablet {
        font-size: 0.9rem; // 14px
        line-height: 1.4rem; //22px
    }
    @include desktop {
        font-size: 0.9rem; // 14px
        line-height: 1.4rem; // 22px 
    }
}

@mixin h4-paragraphs-and-comments-bold {
    @include h4-paragraphs-and-comments;
    font-family: AvenirNextLTProBold;
}

@mixin h4-paragraphs-and-comments-white {
    @include h4-paragraphs-and-comments;
    color: $white;
}

@mixin h4-paragraphs-and-comments-white-bold {
    @include h4-paragraphs-and-comments-bold;
    color: $white;
}

@mixin h4-paragraphs-and-comments-white-demi {
    @include h4-paragraphs-and-comments-white;
    font-family: AvenirNextLTProDemi;
    color: $white;
}

$time: 330ms;
$btns: ( 
alpha: ( gstart: #21D4FD,
gend: #B721FF,
tstart: #B721FF,
tend: #21D4FD),
beta: ( gstart: #08AEEA,
gend: #2AF598,
tstart: #2AF598,
tend: #08AEEA),
gamma: ( gstart: #FEE140,
gend: #FA709A,
tstart: #FA709A,
tend: #FEE140),
delta: ( gstart: #3EECAC,
gend: #EE74E1,
tstart: #EE74E1,
tend: #3EECAC));
.btn {
	outline: none;;
	margin-right: 30px;
	margin-top: 30px;
	font-family: 'Source Sans Pro', sans-serif;
	font-weight: 900;
	padding: 1.25rem 2rem;
	font-size: 1rem;
	border-radius: 3.5rem / 100%;
	position: relative;
	min-width: 15rem;
	max-width: 90vw;
	overflow: hidden;
	border: 0;
	cursor: pointer;
	text-transform: uppercase;
	letter-spacing: 0.05em;
	transition: all $time;
	outline: none;;
	&+& {
		outline: none;;
		margin-top: 1rem;
	}
	span {
		outline: none;;
		position: relative;
		z-index: 1;
	}
	&:before {
		outline: none;;
		content: "";
		background-color: #21D4FD;
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		transform: scale(4) translateX(-100%);
		transition: all $time * 1.5 ease-out;
		outline: none;;
	}
	&:hover,
	&:focus,
	&:active {
		&:before {
			outline: none;;
			transform: scale(4) translate(37%);
		}
	}
}

@each $bname,
$bcolors in $btns {
	.btn--#{$bname} {
		outline: none;;
		background-color: map-get($bcolors, gstart);
		color: map-get($bcolors, tstart);
		&:before {
			background-color: map-get($bcolors, gend);
			background-image: linear-gradient(to right, map-get($bcolors, gend) 30%, map-get($bcolors, gstart) 100%);
			position: absolute;
		}
		&:hover,
		&:focus,
		&:active {
			outline: none;;
			color: map-get($bcolors, tend);
		}
	}
}

.btn-form {
	outline: none;;
	outline: none;;
	margin-right: 30px;
	margin-top: 30px;
	font-family: 'Source Sans Pro', sans-serif;
	font-weight: 900;
	padding: 1.25rem 2rem;
	font-size: 1rem;
	border-radius: 3.5rem / 100%;
	position: relative;
	min-width: 15rem;
	max-width: 90vw;
	overflow: hidden;
	border: 0;
	cursor: pointer;
	text-transform: uppercase;
	letter-spacing: 0.05em;
	transition: all $time;
	 ;
	&+& {
		margin-top: 1rem;
		 ;
	}
	span {
		position: relative;
		z-index: 1;
		 ;
	}
	&:before {
		content: "";
		background-color: #21D4FD;
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		transform: scale(4) translateX(-100%);
		transition: all $time * 1.5 ease-out;
		 ;
	}
	&:hover,
	&:focus,
	&:active {
		&:before {
			 
			//transform: scale(4) translate(37%);
		  //  color: map-get($bcolors, tstart);
		  background-color:#B721FF;;
		}
	}
}

@each $bname,
$bcolors in $btns {
	.btn-form--#{$bname} {
		background-color: map-get($bcolors, gstart);
		color: map-get($bcolors, tstart);
		&:before {
			background-color: map-get($bcolors, gend);
			background-image: linear-gradient(to right, map-get($bcolors, gend) 30%, map-get($bcolors, gstart) 100%);
			position: absolute;
		}
		&:hover,
		&:focus,
		&:active {
		//	color: map-get($bcolors, tstart);
		color: #B721FF;
		}
	}
}
/********************************h4-lists-and-contacts-information*************/

@mixin h4-list-contact-information {
    color: $black;
    font-family: AvenirNextLTProRegular;
    font-size: 0.8rem; //13px 
    line-height: 1.1rem; //18px 
    @include tablet {
        font-size: 0.9rem; //14px 
        line-height: 1.3rem; //20px
    }
    @include desktop {
        font-size: 0.9rem; //14px 
        line-height: 1.3rem; //20px 
    }
}

@mixin h4-list-contact-information-bold {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProBold;
}

@mixin h4-list-contact-information-demi {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProDemi;
}

@mixin h4-list-contact-information-white {
    @include h4-list-contact-information;
    color: $white;
}

@mixin h4-list-contact-information-white-demi {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProDemi;
    color: $white;
}

@mixin h4-list-contact-information-white-bold {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProBold;
    color: $white;
}


//********************************P-Paragraph****************************/

@mixin p-paragraph {
    font-family: AvenirNextLTProRegular;
    color: $black;
    font-size: 0.8rem; //13px
    line-height: 1.1rem; //18px
    @include tablet {
        font-size: 0.9rem; //13px 
        line-height: 1.3rem; //18px 
        display: inline-block;
    }
    @include desktop {
        font-size: 0.9rem; //13px 
        line-height: 1.3rem; //18px
        display: inline-block;
    }
}

@mixin p-paragraph-bold {
    @include p-paragraph;
    font-family: AvenirNextLTProBold;
}

@mixin p-paragraph-white {
    @include p-paragraph;
    color: white;
}

@mixin p-paragraph-white-bold {
    @include p-paragraph-bold;
    color: white;
}

@mixin h1-page-header {
	font-family: TitilliumWebSemiBold600;
	color: $black;
	font-size: 28px; //28px
	line-height: 36px; //36px
	@include tablet {
        font-size: 32px; //32px
        line-height: 40px; //40px
	}
	@include desktop {
        font-size: 32px; //32px
        line-height: 40px; //40px
	}
}

@mixin h2-subheader {
	font-family: TitilliumWebSemiBold600;
	color: $black;
	font-size: 20px; //20px
	line-height: 28px; //28px
	@include tablet {
        font-size: 24px; //24px
        line-height: 32px; //28px
	}
	@include desktop {
        font-size: 24px; //24px
        line-height: 32px; //28px
	}
}
@mixin h3-labels-links-buttons {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 13px; //13px
	line-height: 20px; //20px
	@include tablet {
        font-size: 14px; //14px
        line-height: 22px; //22px
	}
	@include desktop {
        font-size: 14px; //14px
        line-height: 22px; //22px
	}
}
@mixin h4-labels-table-header {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 11px; //11px
	line-height: 16px; //16px
	@include tablet {
        font-size: 12px; //12px
        line-height: 18px; //18px
	}
	@include desktop {
        font-size: 12px; //12px
        line-height: 18px; //18px
	}
}
@mixin p1-body-large {
	font-family: TitilliumWebRegular400;
	color: $black;
    font-size: 15px; //15px
    line-height: 26px; //26px
	@include tablet {
        font-size: 16px; //12px
        line-height: 28px; //18px
	}
	@include desktop {
        font-size: 16px; //12px
        line-height: 28px; //18px
	}
}
@mixin p2-body-medium {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 16px; //13px
	line-height: 28px; //20px
	@include tablet {
        font-size: 13px; //13px
        line-height: 20px; //20px
	}
	@include desktop {
        font-size: 13px; //13px
        line-height: 20px; //20px
	}
}
@mixin p2-body-small {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 11px; //13px
	line-height: 16px; //20px
	@include tablet {
        font-size: 12px; //13px
        line-height: 18px; //20px
	}
	@include desktop {
        font-size: 12px; //13px
        line-height: 18px; //20px
	}
}

/*******************************************H5-Labels***********************/

@mixin h5-labels {
    font-size: 0.8rem; //12px
    line-height: 1.3rem; //20px 
    font-family: AvenirNextLTProRegular;
    color: $silver; //labels
    @include tablet {
        font-size: 0.8rem; //12px
        line-height: 1.3rem; // 20px
        font-family: AvenirNextLTProDemi;
    }
    @include desktop {
        font-size: 0.8rem; //12px
        line-height: 1.8rem; // 20px
    }
}

@mixin h5-labels-demi {
    @include h5-labels;
    font-family: AvenirNextLTProDemi;
}

@mixin h5-labels-bold {
    @include h5-labels;
    font-family: AvenirNextLTProBold;
}


//Breakpoints
@mixin tablet {
    @media (min-width: $breakpoint-tablet) {
        @content;
    }
}

@mixin desktop {
    @media (min-width: $breakpoint-desktop) {
        @content;
    }
}

//Text Styles- Desktop

@mixin d-header {
    font-family: $font-family;
    font-size: 1.875rem;
    line-height: 2.25rem;
    font-weight: 700;
	color: $text-black;
}

@mixin d-subheader {
	font-family: $font-family;
    font-size: 1.875rem;
    line-height: 2.25rem;
    font-weight: 500;
	color: $text-black;
}

@mixin d-subheader-grey {
	font-family: $font-family;
    font-size: 1.75rem;
    line-height: 2rem;
	color: $text-grey;
    font-weight: 500;
}

@mixin d-paragraph-bold {
	font-family: $font-family;
    font-size: 1.125rem;
    line-height: 1.25rem;
	color: $text-black;
    font-weight: 700;
}

@mixin d-paragraph-large {
	font-family: $font-family;
    font-size: 1.375rem;
    line-height: 1.4rem;
	color: $text-grey;
    font-weight: 400;
}

@mixin d-paragraph-medium {
	font-family: $font-family;
    font-size: 1.25rem;
    line-height: 1.4rem;
	color: $text-grey;
    font-weight: 400;
}

@mixin d-paragraph-small {
	font-family: $font-family;
    font-size: 1.4rem;
    line-height: 1rem;
	color: $text-grey;
    font-weight: 400;
}

//Button Styling
@mixin button-blue {
    background-color: $secondary-color-indigo;
    border-radius: 5px;
    color: white;
    padding: 0.875rem 1.813rem;
    appearance: none;
    border: none;
}

//Component Styling

@mixin component-styling {
    box-shadow: 0px 10px 25px rgba(19, 24, 44, 0.1);
    border-radius: 10px;
    padding: 3.75rem;
}
/***************************************STANDARD PADDING**************************************************/
@mixin standard-padding-right {
	padding-right: $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding-right: $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding-right: $mockup-column-outside-desktop; //80px right 80px left
	}
  }

  @mixin standard-padding-left {
	padding-left: $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding-left: $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding-left: $mockup-column-outside-desktop; //80px right 80px left
	}
  }
  
  @mixin standard-padding-horizontal {
	padding: 0 $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding: 0 $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding: 0 $mockup-column-outside-desktop; //80px right 80px left
	}
  }
  
  
  /**********************************WIDTH*100%*PADDING**********************************************/
  
  @mixin width-100-percent-minus-standard-padding-horizontal {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent} * 2));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent} * 2));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent} * 2));
	}
  }
  
  @mixin width-100-percent-minus-standard-padding-left {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent}));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent}));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent}));
	}
  }
  
  @mixin width-100-percent-minus-standard-padding-right {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent}));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent}));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent}));
	}
  }
  

  /*******************WIDTH MINUS MOCKUP COLUMN************/
  @mixin width-100-percent-minus-mockup-column-outside {
	width: calc(100% - #{$mockup-column-outside-mobile});
	@include tablet {
	  width: calc(100% - #{$mockup-column-outside-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-outside-desktop});
	}
  }
  @mixin width-100-percent-minus-mockup-column-outside-horizontal {
	width: calc(100% - (#{$mockup-column-outside-mobile} * 2));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet} *2));
	}
	@include desktop {
	  width: calc(100% - (#{mockup-column-outside-desktop} *2));
	}
  }
  @mixin width-100-percent-minus-mockup-column-small {
	width: calc(100% - #{mockup-column-small-mobile});
	@include tablet {
	  width: calc(100% -#{mockup-column-small-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-small-desktop});
	}
  }
  
  @mixin width-100-percent-minus-mockup-column-large {
	width: calc(100% - #{mockup-column-large-mobile});
	@include tablet {
	  width: calc(100% -#{mockup-column-large-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-large-desktop});
	}
  }
  



@mixin fancybutton {
  font-family: 'Source Sans Pro', sans-serif;
  font-weight: 900;
  padding: 1.25rem 2rem;
  font-size: 1rem;
  border-radius: 3.5rem / 100%;
  position: relative;
  min-width: 15rem;
  max-width: 90vw;
  overflow: hidden;
  border: 0;
  cursor: pointer;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  transition: all $time;
  
  & + & {
    margin-top: 1rem;
  }
  
  span {
    position: relative;
    z-index: 1;
  }
  
  &:before {
    content: "";
    background-color: #21D4FD;
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    transform: scale(4) translateX(-100%);
    transition: all $time * 1.5 ease-out;
  }
  
  &:hover,
  &:focus,
  &:active {    
    &:before {
      transform: scale(4) translate(37%);
    } 
  }
}


  /**********************************************DIVIDERS******************************************************/
  
  @mixin divider {
	background: $silver;
	width: 100%;
	height: 1px;
  }
  
  @mixin divider-form {
	background: $silver;
	width: 100%;
	height: 1px;
	margin-bottom: $increment2-px;
  }
  
  /**********************************************LOGOS******************************************************/
  
  @mixin logo-brainflix {
	img {
	  width: 120px; //120px
	  height: 24px; //24px
	  margin-left: 1rem;
	  @include tablet {
		margin-left: 2rem;
		width: 120px; //120px
		height: 24px; //24px
	  }
	  @include desktop {
		margin-left: 5rem;
	  }
	}
  }
  
  /**********************************************BUTTONS******************************************************/
  
  @mixin button {
	background-color: $brainflix-blue;
	@include h4-paragraphs-and-comments-white-demi;
	&:hover {
	  background: $brainflix-dark-blue-hover;
	}
	&:focus {
	  outline: none;
	}
	width: $increment28-px; //226px
	height: $increment6-px; 
	border-radius: $increment0-px; 
	border-color: $brainflix-blue;
	@include tablet {
	  padding: 0;
	  width: $increment20-px; //160px
	  height: $increment6-px; 
	}
  }
  
  /**********************************************MOCKUP COLUMNS****************************************************************/
  
  /*******************RIGHT*****************/
  
  @mixin mockup-column-outside-right {
	margin-right: $mockup-column-outside-mobile;
	@include tablet {
	  margin-right: $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-right {
	margin-right: $mockup-column-small-mobile;
	@include tablet {
	  margin-right: $mockup-column-small-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-right {
	margin-right: $mockup-column-large-mobile;
	@include tablet {
	  margin-right: $mockup-column-large-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-large-desktop;
	}
  }
  
  /*******************LEFT***************/
  
  @mixin mockup-column-outside-left {
	margin-left: $mockup-column-outside-mobile;
	@include tablet {
	  margin-left: $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-left {
	margin-left: $mockup-column-small-mobile;
	@include tablet {
	  margin-left: $mockup-column-small-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-left {
	margin-left: $mockup-column-large-mobile;
	@include tablet {
	  margin-left: $mockup-column-large-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-large-desktop;
	}
  }
  
  /*********************HORIZONTAL**************/
  
  @mixin mockup-column-outside-horizontal {
	margin: 0 $mockup-column-outside-mobile;
	@include tablet {
	  margin: 0 $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-horizontal {
	margin: 0 $mockup-column-small-mobile;
	@include tablet {
	  margin: 0 $mockup-column-small-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-horizontal {
	margin: 0 $mockup-column-large-mobile;
	@include tablet {
	  margin: 0 $mockup-column-large-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-large-desktop;
	}
  }
  
  /*********************************************WIDTH 100% MINUS INCREMENTS*********************************/
  
  @mixin width-100-percent-minus-increment0 {
	width: calc(100% - #{increment0-mobile});
	@include tablet {
	  width: calc(100% -#{increment0-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment0-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment1 {
	width: calc(100% - #{increment1-mobile});
	@include tablet {
	  width: calc(100% - #{increment1-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment1-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment2 {
	width: calc(100% - #{increment2-mobile});
	@include tablet {
	  width: calc(100% -#{increment2-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment2-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment3 {
	width: calc(100% - #{increment3-mobile});
	@include tablet {
	  width: calc(100% -#{increment3-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment3-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment4 {
	width: calc(100% - #{increment4-mobile});
	@include tablet {
	  width: calc(100% -#{increment4-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment4-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment5 {
	width: calc(100% - #{increment5-mobile});
	@include tablet {
	  width: calc(100% -#{increment5-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment5-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment6 {
	width: calc(100% - #{increment6-mobile});
	@include tablet {
	  width: calc(100% -#{increment6-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment6-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment7 {
	width: calc(100% - #{increment7-mobile});
	@include tablet {
	  width: calc(100% -#{increment7-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment7-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment8 {
	width: calc(100% - #{increment8-mobile});
	@include tablet {
	  width: calc(100% -#{increment8-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment8-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment9 {
	width: calc(100% - #{increment9-mobile});
	@include tablet {
	  width: calc(100% -#{increment9-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment9-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment10 {
	width: calc(100% - #{increment10-mobile});
	@include tablet {
	  width: calc(100% -#{increment10-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment10-desktop});
	}
  }
  

  
  /************************************%%%%%%%%%%%%%%%%%%%%%******************************************************
  /******%%%%%%%%%%%%%%%%%%%%%%%****CONTENTS LIST:*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****TOP MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *****%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%******RIGHT MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***BOTTOM MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***LEFT MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ***********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*VERTICAL MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ***********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*HORIZTONAL**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***WIDTH **********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%****HEIGHT**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****OTHER COMMON INCREMENTS*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
   ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****MOCKUP COLUMN PADDING*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********/
  
  /*******************************************************INCREMENT MARGIN TOP******************************************/
  
  @mixin increment-0-top {
	margin-top: $increment0-mobile;
	@include tablet {
	  margin-top: $increment0-tablet;
	}
	@include desktop {
	  margin-top: $increment0-desktop;
	}
  }
  
  @mixin increment-1-top {
	margin-top: $increment1-mobile;
	@include tablet {
	  margin-top: $increment1-tablet;
	}
	@include desktop {
	  margin-top: $increment1-desktop;
	}
  }
  
  @mixin increment-2-top {
	margin-top: $increment2-mobile;
	@include tablet {
	  margin-top: $increment2-tablet;
	}
	@include desktop {
	  margin-top: $increment2-desktop;
	}
  }
  
  @mixin increment-3-top {
	margin-top: $increment3-mobile;
	@include tablet {
	  margin-top: $increment3-tablet;
	}
	@include desktop {
	  margin-top: $increment3-desktop;
	}
  }
  
  @mixin increment-4-top {
	margin-top: $increment4-mobile;
	@include tablet {
	  margin-top: $increment4-tablet;
	}
	@include desktop {
	  margin-top: $increment4-desktop;
	}
  }
  
  @mixin increment-5-top {
	margin-top: $increment5-mobile;
	@include tablet {
	  margin-top: $increment5-tablet;
	}
	@include desktop {
	  margin-top: $increment5-desktop;
	}
  }
  
  @mixin increment-6-top {
	margin-top: $increment6-mobile;
	@include tablet {
	  margin-top: $increment6-tablet;
	}
	@include desktop {
	  margin-top: $increment6-desktop;
	}
  }
  
  @mixin increment-7-top {
	margin-top: $increment7-mobile;
	@include tablet {
	  margin-top: $increment7-tablet;
	}
	@include desktop {
	  margin-top: $increment7-desktop;
	}
  }
  
  @mixin increment-8-top {
	margin-top: $increment8-mobile;
	@include tablet {
	  margin-top: $increment8-tablet;
	}
	@include desktop {
	  margin-top: $increment8--desktop;
	}
  }
  
  @mixin increment-9-top {
	margin-top: $increment9-mobile;
	@include tablet {
	  margin-top: $increment9-tablet;
	}
	@include desktop {
	  margin-top: $increment9--desktop;
	}
  }
  
  @mixin increment-10-top {
	margin-top: $increment10-mobile;
	@include tablet {
	  margin-top: $increment10-tablet;
	}
	@include desktop {
	  margin-top: $increment10--desktop;
	}
  }
  
  /***********************************INCREMENT RIGHT*************************************************/
  
  @mixin increment-0-right {
	margin-right: $increment0-mobile;
	@include tablet {
	  margin-right: $increment0-tablet;
	}
	@include desktop {
	  margin-right: $increment0-desktop;
	}
  }
  
  @mixin increment-1-right {
	margin-right: $increment1-mobile;
	@include tablet {
	  margin-right: $increment1-tablet;
	}
	@include desktop {
	  margin-right: $increment1-desktop;
	}
  }
  
  @mixin increment-2-right {
	margin-right: $increment2-mobile;
	@include tablet {
	  margin-right: $increment2-tablet;
	}
	@include desktop {
	  margin-right: $increment2-desktop;
	}
  }
  
  @mixin increment-3-right {
	margin-right: $increment3-mobile;
	@include tablet {
	  margin-right: $increment3-tablet;
	}
	@include desktop {
	  margin-right: $increment3-desktop;
	}
  }
  
  @mixin increment-4-right {
	margin-right: $increment4-mobile;
	@include tablet {
	  margin-right: $increment4-tablet;
	}
	@include desktop {
	  margin-right: $increment4-desktop;
	}
  }
  
  @mixin increment-5-right {
	margin-right: $increment5-mobile;
	@include tablet {
	  margin-right: $increment5-tablet;
	}
	@include desktop {
	  margin-right: $increment5-desktop;
	}
  }
  
  @mixin increment-6-right {
	margin-right: $increment6-mobile;
	@include tablet {
	  margin-right: $increment6-tablet;
	}
	@include desktop {
	  margin-right: $increment6-desktop;
	}
  }
  
  @mixin increment-7-right {
	margin-right: $increment7-mobile;
	@include tablet {
	  margin-right: $increment7-tablet;
	}
	@include desktop {
	  margin-right: $increment7-desktop;
	}
  }
  
  @mixin increment-8-right {
	margin-right: $increment8-mobile;
	@include tablet {
	  margin-right: $increment8-tablet;
	}
	@include desktop {
	  margin-right: $increment8--desktop;
	}
  }
  
  @mixin increment-9-right {
	margin-right: $increment9-mobile;
	@include tablet {
	  margin-right: $increment9-tablet;
	}
	@include desktop {
	  margin-right: $increment9--desktop;
	}
  }
  
  @mixin increment-10-right {
	margin-right: $increment10-mobile;
	@include tablet {
	  margin-right: $increment10-tablet;
	}
	@include desktop {
	  margin-right: $increment10--desktop;
	}
  }
  
  /*******************************************************INCREMENT MARGIN BOTTOM******************************************/
  
  @mixin increment-0-bottom {
	margin-bottom: $increment0-mobile;
	@include tablet {
	  margin-bottom: $increment0-tablet;
	}
	@include desktop {
	  margin-bottom: $increment0-desktop;
	}
  }
  
  @mixin increment-1-bottom {
	margin-bottom: $increment1-mobile;
	@include tablet {
	  margin-bottom: $increment1-tablet;
	}
	@include desktop {
	  margin-bottom: $increment1-desktop;
	}
  }
  
  @mixin increment-2-bottom {
	margin-bottom: $increment2-mobile;
	@include tablet {
	  margin-bottom: $increment2-tablet;
	}
	@include desktop {
	  margin-bottom: $increment2-desktop;
	}
  }
  
  @mixin increment-3-bottom {
	margin-bottom: $increment3-mobile;
	@include tablet {
	  margin-bottom: $increment3-tablet;
	}
	@include desktop {
	  margin-bottom: $increment3-desktop;
	}
  }
  
  @mixin increment-4-bottom {
	margin-bottom: $increment4-mobile;
	@include tablet {
	  margin-bottom: $increment4-tablet;
	}
	@include desktop {
	  margin-bottom: $increment4-desktop;
	}
  }
  
  @mixin increment-5-bottom {
	margin-bottom: $increment5-mobile;
	@include tablet {
	  margin-bottom: $increment5-tablet;
	}
	@include desktop {
	  margin-bottom: $increment5-desktop;
	}
  }
  
  @mixin increment-6-bottom {
	margin-bottom: $increment6-mobile;
	@include tablet {
	  margin-bottom: $increment6-tablet;
	}
	@include desktop {
	  margin-bottom: $increment6-desktop;
	}
  }
  
  @mixin increment-7-bottom {
	margin-bottom: $increment7-mobile;
	@include tablet {
	  margin-bottom: $increment7-tablet;
	}
	@include desktop {
	  margin-bottom: $increment7-desktop;
	}
  }
  
  @mixin increment-8-bottom {
	margin-bottom: $increment8-mobile;
	@include tablet {
	  margin-bottom: $increment8-tablet;
	}
	@include desktop {
	  margin-bottom: $increment8--desktop;
	}
  }
  
  @mixin increment-9-bottom {
	margin-bottom: $increment9-mobile;
	@include tablet {
	  margin-bottom: $increment9-tablet;
	}
	@include desktop {
	  margin-bottom: $increment9--desktop;
	}
  }
  
  @mixin increment-10-bottom {
	margin-bottom: $increment10-mobile;
	@include tablet {
	  margin-bottom: $increment10-tablet;
	}
	@include desktop {
	  margin-bottom: $increment10--desktop;
	}
  }
  
  /*******************************************************INCREMENT LEFT******************************************/
  
  @mixin increment-0-left {
	margin-left: $increment0-mobile;
	@include tablet {
	  margin-left: $increment0-tablet;
	}
	@include desktop {
	  margin-left: $increment0-desktop;
	}
  }
  
  @mixin increment-1-left {
	margin-left: $increment1-mobile;
	@include tablet {
	  margin-left: $increment1-tablet;
	}
	@include desktop {
	  margin-left: $increment1-desktop;
	}
  }
  
  @mixin increment-2-left {
	margin-left: $increment2-mobile;
	@include tablet {
	  margin-left: $increment2-tablet;
	}
	@include desktop {
	  margin-left: $increment2-desktop;
	}
  }
  
  @mixin increment-3-left {
	margin-left: $increment3-mobile;
	@include tablet {
	  margin-left: $increment3-tablet;
	}
	@include desktop {
	  margin-left: $increment3-desktop;
	}
  }
  
  @mixin increment-4-left {
	margin-left: $increment4-mobile;
	@include tablet {
	  margin-left: $increment4-tablet;
	}
	@include desktop {
	  margin-left: $increment4-desktop;
	}
  }
  
  @mixin increment-5-left {
	margin-left: $increment5-mobile;
	@include tablet {
	  margin-left: $increment5-tablet;
	}
	@include desktop {
	  margin-left: $increment5-desktop;
	}
  }
  
  @mixin increment-6-left {
	margin-left: $increment6-mobile;
	@include tablet {
	  margin-left: $increment6-tablet;
	}
	@include desktop {
	  margin-left: $increment6-desktop;
	}
  }
  
  @mixin increment-7-left {
	margin-left: $increment7-mobile;
	@include tablet {
	  margin-left: $increment7-tablet;
	}
	@include desktop {
	  margin-left: $increment7-desktop;
	}
  }
  
  @mixin increment-8-left {
	margin-left: $increment8-mobile;
	@include tablet {
	  margin-left: $increment8-tablet;
	}
	@include desktop {
	  margin-left: $increment8--desktop;
	}
  }
  
  @mixin increment-9-left {
	margin-left: $increment9-mobile;
	@include tablet {
	  margin-left: $increment9-tablet;
	}
	@include desktop {
	  margin-left: $increment9--desktop;
	}
  }
  
  @mixin increment-10-left {
	margin-left: $increment10-mobile;
	@include tablet {
	  margin-left: $increment10-tablet;
	}
	@include desktop {
	  margin-left: $increment10--desktop;
	}
  }
  
  /************************************VERTICAL MARGIN INCREMENTS******************************************************/
  
  @mixin increment-0-vertical {
	margin: $increment0-mobile 0;
	@include tablet {
	  margin: $increment0-tablet 0;
	}
	@include deskvertical {
	  margin: $increment0-desktop 0;
	}
  }
  
  @mixin increment-1-vertical {
	margin: $increment1-mobile 0;
	@include tablet {
	  margin: $increment1-tablet 0;
	}
	@include desktop {
	  margin: $increment1-desktop 0;
	}
  }
  
  @mixin increment-2-vertical {
	margin: $increment2-mobile 0;
	@include tablet {
	  margin: $increment2-tablet 0;
	}
	@include desktop {
	  margin: $increment2-desktop 0;
	}
  }
  
  @mixin increment-3-vertical {
	margin: $increment3-mobile 0;
	@include tablet {
	  margin: $increment3-tablet 0;
	}
	@include desktop {
	  margin: $increment3-desktop 0;
	}
  }
  
  @mixin increment-4-vertical {
	margin: $increment4-mobile 0;
	@include tablet {
	  margin: $increment4-tablet 0;
	}
	@include desktop {
	  margin: $increment4-desktop 0;
	}
  }
  
  @mixin increment-5-vertical {
	margin: $increment5-mobile 0;
	@include tablet {
	  margin: $increment5-tablet 0;
	}
	@include desktop {
	  margin: $increment5-desktop 0;
	}
  }
  
  @mixin increment-6-vertical {
	margin: $increment6-mobile 0;
	@include tablet {
	  margin: $increment6-tablet 0;
	}
	@include desktop {
	  margin: $increment6-desktop 0;
	}
  }
  
  @mixin increment-7-vertical {
	margin: $increment7-mobile 0;
	@include tablet {
	  margin: $increment7-tablet 0;
	}
	@include desktop {
	  margin: $increment7-desktop 0;
	}
  }
  
  @mixin increment-8-vertical {
	margin: $increment8-mobile 0;
	@include tablet {
	  margin: $increment8-tablet 0;
	}
	@include desktop {
	  margin: $increment8--desktop 0;
	}
  }
  
  @mixin increment-9-vertical {
	margin: $increment9-mobile 0;
	@include tablet {
	  margin: $increment9-tablet 0;
	}
	@include desktop {
	  margin: $increment9--desktop 0;
	}
  }
  
  @mixin increment-10-vertical {
	margin: $increment10-mobile 0;
	@include tablet {
	  margin: $increment10-tablet 0;
	}
	@include desktop {
	  margin: $increment10--desktop 0;
	}
  }
  
  /************************************HORIZONTAL MARGIN INCREMENTS******************************************************/
  
  @mixin increment-0-horizontal {
	margin: 0 $increment0-mobile;
	@include tablet {
	  margin: 0 $increment0-tablet;
	}
	@include deskhorizontal {
	  margin: 0 $increment0-desktop;
	}
  }
  
  @mixin increment-1-horizontal {
	margin: 0 $increment1-mobile;
	@include tablet {
	  margin: 0 $increment1-tablet;
	}
	@include desktop {
	  margin: 0 $increment1-desktop;
	}
  }
  
  @mixin increment-2-horizontal {
	margin: 0 $increment2-mobile;
	@include tablet {
	  margin: 0 $increment2-tablet;
	}
	@include desktop {
	  margin: 0 $increment2-desktop;
	}
  }
  
  @mixin increment-3-horizontal {
	margin: 0 $increment3-mobile;
	@include tablet {
	  margin: 0 $increment3-tablet;
	}
	@include desktop {
	  margin: 0 $increment3-desktop;
	}
  }
  
  @mixin increment-4-horizontal {
	margin: 0 $increment4-mobile;
	@include tablet {
	  margin: 0 $increment4-tablet;
	}
	@include desktop {
	  margin: 0 $increment4-desktop;
	}
  }
  
  @mixin increment-5-horizontal {
	margin: 0 $increment5-mobile;
	@include tablet {
	  margin: 0 $increment5-tablet;
	}
	@include desktop {
	  margin: 0 $increment5-desktop;
	}
  }
  
  @mixin increment-6-horizontal {
	margin: 0 $increment6-mobile;
	@include tablet {
	  margin: 0 $increment6-tablet;
	}
	@include desktop {
	  margin: 0 $increment6-desktop;
	}
  }
  
  @mixin increment-7-horizontal {
	margin: 0 $increment7-mobile;
	@include tablet {
	  margin: 0 $increment7-tablet;
	}
	@include desktop {
	  margin: 0 $increment7-desktop;
	}
  }
  
  @mixin increment-8-horizontal {
	margin: 0 $increment8-mobile;
	@include tablet {
	  margin: 0 $increment8-tablet;
	}
	@include desktop {
	  margin: 0 $increment8--desktop;
	}
  }
  
  @mixin increment-9-horizontal {
	margin: 0 $increment9-mobile;
	@include tablet {
	  margin: 0 $increment9-tablet;
	}
	@include desktop {
	  margin: 0 $increment9--desktop;
	}
  }
  
  @mixin increment-10-horizontal {
	margin: 0 $increment10-mobile;
	@include tablet {
	  margin: 0 $increment10-tablet;
	}
	@include desktop {
	  margin: 0 $increment10--desktop;
	}
  }
    /***************************************************INCREMENT WIDTH******************************************/
  
	@mixin increment-0-width {
		width: $increment0-mobile;
		@include tablet {
		  width: $increment0-tablet;
		}
		@include deskwidth {
		  width: $increment0-desktop;
		}
	  }
	  
	  @mixin increment-1-width {
		width: $increment1-mobile;
		@include tablet {
		  width: $increment1-tablet;
		}
		@include desktop {
		  width: $increment1-desktop;
		}
	  }
	  
	  @mixin increment-2-width {
		width: $increment2-mobile;
		@include tablet {
		  width: $increment2-tablet;
		}
		@include desktop {
		  width: $increment2-desktop;
		}
	  }
	  
	  @mixin increment-3-width {
		width: $increment3-mobile;
		@include tablet {
		  width: $increment3-tablet;
		}
		@include desktop {
		  width: $increment3-desktop;
		}
	  }
	  
	  @mixin increment-4-width {
		width: $increment4-mobile;
		@include tablet {
		  width: $increment4-tablet;
		}
		@include desktop {
		  width: $increment4-desktop;
		}
	  }
	  
	  @mixin increment-5-width {
		width: $increment5-mobile;
		@include tablet {
		  width: $increment5-tablet;
		}
		@include desktop {
		  width: $increment5-desktop;
		}
	  }
	  
	  @mixin increment-6-width {
		width: $increment6-mobile;
		@include tablet {
		  width: $increment6-tablet;
		}
		@include desktop {
		  width: $increment6-desktop;
		}
	  }
	  
	  @mixin increment-7-width {
		width: $increment7-mobile;
		@include tablet {
		  width: $increment7-tablet;
		}
		@include desktop {
		  width: $increment7-desktop;
		}
	  }
	  
	  @mixin increment-8-width {
		width: $increment8-mobile;
		@include tablet {
		  width: $increment8-tablet;
		}
		@include desktop {
		  width: $increment8--desktop;
		}
	  }
	  
	  @mixin increment-9-width {
		width: $increment9-mobile;
		@include tablet {
		  width: $increment9-tablet;
		}
		@include desktop {
		  width: $increment9--desktop;
		}
	  }
	  
	  @mixin increment-10-width {
		width: $increment10-mobile;
		@include tablet {
		  width: $increment10-tablet;
		}
		@include desktop {
		  width: $increment10--desktop;
		}
	  }
  
  /***************************************************INCREMENT HEIGHT******************************************/
  
  @mixin increment-0-height {
	height: $increment0-mobile;
	@include tablet {
	  height: $increment0-tablet;
	}
	@include deskheight {
	  height: $increment0-desktop;
	}
  }
  
  @mixin increment-1-height {
	height: $increment1-mobile;
	@include tablet {
	  height: $increment1-tablet;
	}
	@include desktop {
	  height: $increment1-desktop;
	}
  }
  
  @mixin increment-2-height {
	height: $increment2-mobile;
	@include tablet {
	  height: $increment2-tablet;
	}
	@include desktop {
	  height: $increment2-desktop;
	}
  }
  
  @mixin increment-3-height {
	height: $increment3-mobile;
	@include tablet {
	  height: $increment3-tablet;
	}
	@include desktop {
	  height: $increment3-desktop;
	}
  }
  
  @mixin increment-4-height {
	height: $increment4-mobile;
	@include tablet {
	  height: $increment4-tablet;
	}
	@include desktop {
	  height: $increment4-desktop;
	}
  }
  
  @mixin increment-5-height {
	height: $increment5-mobile;
	@include tablet {
	  height: $increment5-tablet;
	}
	@include desktop {
	  height: $increment5-desktop;
	}
  }
  
  @mixin increment-6-height {
	height: $increment6-mobile;
	@include tablet {
	  height: $increment6-tablet;
	}
	@include desktop {
	  height: $increment6-desktop;
	}
  }
  
  @mixin increment-7-height {
	height: $increment7-mobile;
	@include tablet {
	  height: $increment7-tablet;
	}
	@include desktop {
	  height: $increment7-desktop;
	}
  }
  
  @mixin increment-8-height {
	height: $increment8-mobile;
	@include tablet {
	  height: $increment8-tablet;
	}
	@include desktop {
	  height: $increment8--desktop;
	}
  }
  
  @mixin increment-9-height {
	height: $increment9-mobile;
	@include tablet {
	  height: $increment9-tablet;
	}
	@include desktop {
	  height: $increment9--desktop;
	}
  }
  
  @mixin increment-10-height {
	height: $increment10-mobile;
	@include tablet {
	  height: $increment10-tablet;
	}
	@include desktop {
	  height: $increment10--desktop;
	}
  }
  
  /***************************************************OTHER COMMON INCREMENTS********************************/
  
  /*******TOP****/
  
  @mixin increment20px-top {
	margin-top: $increment20px-mobile;
	@include tablet {
	  margin-top: $increment20px-tablet;
	}
	@include desktop {
	  margin-top: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-top {
	margin-top: $increment54px-mobile;
	@include tablet {
	  margin-top: $increment54px-tablet;
	}
	@include desktop {
	  margin-top: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-top {
	margin-top: $increment68px-mobile;
	@include tablet {
	  margin-top: $increment68px-tablet;
	}
	@include desktop {
	  margin-top: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-top {
	margin-top: $increment100px-mobile;
	@include tablet {
	  margin-top: $increment100px-tablet;
	}
	@include desktop {
	  margin-top: $increment100px-desktop;
	}
  }
  
  /***************RIGHT********************/
  
  @mixin increment20px-right {
	margin-right: $increment20px-mobile;
	@include tablet {
	  margin-right: $increment20px-tablet;
	}
	@include desktop {
	  margin-right: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-right {
	margin-right: $increment54px-mobile;
	@include tablet {
	  margin-right: $increment54px-tablet;
	}
	@include desktop {
	  margin-right: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-right {
	margin-right: $increment68px-mobile;
	@include tablet {
	  margin-right: $increment68px-tablet;
	}
	@include desktop {
	  margin-right: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-right {
	margin-right: $increment100px-mobile;
	@include tablet {
	  margin-right: $increment100px-tablet;
	}
	@include desktop {
	  margin-right: $increment100px-desktop;
	}
  }
  
  /****************BOTTOM***************/
  
  @mixin increment20px-bottom {
	margin-bottom: $increment20px-mobile;
	@include tablet {
	  margin-bottom: $increment20px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-bottom {
	margin-bottom: $increment54px-mobile;
	@include tablet {
	  margin-bottom: $increment54px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-bottom {
	margin-bottom: $increment68px-mobile;
	@include tablet {
	  margin-bottom: $increment68px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-bottom {
	margin-bottom: $increment100px-mobile;
	@include tablet {
	  margin-bottom: $increment100px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment100px-desktop;
	}
  }
  
  /*****************LEFT****************/
  
  @mixin increment20px-left {
	margin-left: $increment20px-mobile;
	@include tablet {
	  margin-left: $increment20px-tablet;
	}
	@include desktop {
	  margin-left: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-left {
	margin-left: $increment54px-mobile;
	@include tablet {
	  margin-left: $increment54px-tablet;
	}
	@include desktop {
	  margin-left: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-left {
	margin-left: $increment68px-mobile;
	@include tablet {
	  margin-left: $increment68px-tablet;
	}
	@include desktop {
	  margin-left: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-left {
	margin-left: $increment100px-mobile;
	@include tablet {
	  margin-left: $increment100px-tablet;
	}
	@include desktop {
	  margin-left: $increment100px-desktop;
	}
  }
  
  /************VERTICAL***********/
  
  @mixin increment20px-vertical {
	margin: $increment20px-mobile 0;
	@include tablet {
	  margin: $increment20px-tablet 0;
	}
	@include desktop {
	  margin: $increment20px-desktop 0;
	}
  }
  
  @mixin increment54px-vertical {
	margin: $increment54px-mobile 0;
	@include tablet {
	  margin: $increment54px-tablet 0;
	}
	@include desktop {
	  margin: $increment54px-desktop 0;
	}
  }
  
  @mixin increment68px-vertical {
	margin: $increment68px-mobile 0;
	@include tablet {
	  margin: $increment68px-tablet 0;
	}
	@include desktop {
	  margin: $increment68px-desktop 0;
	}
  }
  
  @mixin increment100px-vertical {
	margin: $increment100px-mobile 0;
	@include tablet {
	  margin: $increment100px-tablet 0;
	}
	@include desktop {
	  margin: $increment100px-desktop 0;
	}
  }
  
  /**********HORIZONTAL*********/
  
  @mixin increment20px-horizontal {
	margin: 0 $increment20px-mobile;
	@include tablet {
	  margin: 0 $increment20px-tablet;
	}
	@include desktop {
	  margin: 0 $increment20px-desktop;
	}
  }
  
  @mixin increment54px-horizontal {
	margin: 0 $increment54px-mobile;
	@include tablet {
	  margin: 0 $increment54px-tablet;
	}
	@include desktop {
	  margin: 0 $increment54px-desktop;
	}
  }
  
  @mixin increment68px-horizontal {
	margin: 0 $increment68px-mobile;
	@include tablet {
	  margin: 0 $increment68px-tablet;
	}
	@include desktop {
	  margin: 0 $increment68px-desktop;
	}
  }
  
  @mixin increment100px-horizontal {
	margin: 0 $increment100px-mobile;
	@include tablet {
	  margin: 0 $increment100px-tablet;
	}
	@include desktop {
	  margin: 0 $increment100px-desktop;
	}
  }
  
 /***********WIDTH*********/
  
  @mixin increment20px-width {
	width: $increment20px-mobile;
	@include tablet {
	  width: $increment20px-tablet;
	}
	@include desktop {
	  width: $increment20px-desktop;
	}
  }

 @mixin increment54px-width {
	width: $increment54px-mobile;
	@include tablet {
	  width: $increment54px-tablet;
	}
	@include desktop {
	  width: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-width {
	width: $increment68px-mobile;
	@include tablet {
	  width: $increment68px-tablet;
	}
	@include desktop {
	  width: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-width {
	width: $increment100px-mobile;
	@include tablet {
	  width: $increment100px-tablet;
	}
	@include desktop {
	  width: $increment100px-desktop;
	}
  }

  /***********HEIGHT*********/
  @mixin increment20px-height {
	height: $increment20px-mobile;
	@include tablet {
	  height: $increment20px-tablet;
	}
	@include desktop {
	  height: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-height {
	height: $increment54px-mobile;
	@include tablet {
	  height: $increment54px-tablet;
	}
	@include desktop {
	  height: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-height {
	height: $increment68px-mobile;
	@include tablet {
	  height: $increment68px-tablet;
	}
	@include desktop {
	  height: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-height {
	height: $increment100px-mobile;
	@include tablet {
	  height: $increment100px-tablet;
	}
	@include desktop {
	  height: $increment100px-desktop;
	}
  }
  
  /**********************************************MOCKUP COLUMNS PADDING****************************************************************/
  
  @mixin mockup-column-padding-outside-right {
	padding-right: $mockup-column-outside-mobile;
	@include tablet {
	  padding-right: $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-right {
	padding-right: $mockup-column-small-mobile;
	@include tablet {
	  padding-right: $mockup-column-small-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-right {
	padding-right: $mockup-column-large-mobile;
	@include tablet {
	  padding-right: $mockup-column-large-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-large-desktop;
	}
  }
  
  /***LEFT*******/
  
  @mixin mockup-column-padding-outside-left {
	padding-left: $mockup-column-outside-mobile;
	@include tablet {
	  padding-left: $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-left {
	padding-left: $mockup-column-small-mobile;
	@include tablet {
	  padding-left: $mockup-column-small-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-left {
	padding-left: $mockup-column-large-mobile;
	@include tablet {
	  padding-left: $mockup-column-large-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-large-desktop;
	}
  }
  
  /***HORIZONTAL*******/
  
  @mixin mockup-column-padding-outside-horizontal {
	padding: 0 $mockup-column-outside-mobile;
	@include tablet {
	  padding: 0 $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-horizontal {
	padding: 0 $mockup-column-small-mobile;
	@include tablet {
	  padding: 0 $mockup-column-small-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-horizontal {
	padding: 0 $mockup-column-large-mobile;
	@include tablet {
	  padding: 0 $mockup-column-large-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-large-desktop;
	}
  }
  /*************************************TEXT STYLES************************************/
@use "./colors" as *;
@use "./breakpoints" as *;
@use "./variables" as *;
/*****************************************H1-Main-heading***************************/

@mixin h1-main-header {
    font-family: AvenirNextLTProBold;
    font-size: 1.5rem; //24px 
    line-height: 2.0rem; //32px
    color: $white;
    @include tablet {
        font-size: 2.3rem; //36px 2.25rem 
        line-height: 2.8rem; //44px
    }
    @include desktop {
        font-size: 2.3rem; //36px 2.25rem 
        line-height: 2.8rem; //44px
    }
}

@mixin h1-main-header-regular {
    @include h1-main-header;
    font-family: AvenirNextLTProRegular;
}

@mixin h1-main-header-black {
    @include h1-main-header;
    color: $black;
}

@mixin h1-desktop-size-that-should-be-available-on-mobile {
    color: $white;
    font-family: AvenirNextLTProBold;
    font-size: 8.0vw;
    line-height: 10.8vw;
}


/*********************H2 - Header subheader for mobile*********************/

@mixin h2-header {
    font-size: 0.9rem; //14px 
    line-height: 1.4rem; //22px 
    font-family: AvenirNextLTProDemi;
    color: $black;
    @include tablet {
        font-size: 1.5rem; //24px 1.5rem 
        line-height: 2.0; //32px 2rem
        font-family: AvenirNextLTProBold;
    }
    @include desktop {
        font-size: 1.5rem; //24px 1.5rem 
        line-height: 2.0rem; //32px 2rem 
    }
}

@mixin h2-header-white {
    @include h2-header;
    color: $white;
}

@mixin h2-header-responsive-white {
    @include h2-header-responsive;
    color: $white;
}

@mixin h2-desktop-size-that-should-be-available-on-mobile {
    color: $white;
    font-size: 1.5rem; //24px 1.5rem 
    line-height: 2.0; //32px 2rem
    font-family: AvenirNextLTProBold;
}


/*******************************H3 - Subheader for Tablet and Desktop************/

@mixin h3-subheader {
    font-size: 1.0rem; //16px 
    line-height: 1.5rem; // 24px
    font-family: AvenirNextLTProDemi;
    color: $black;
    @include tablet {
        font-size: 1.0rem; //16px 
        line-height: 1.5rem; // 24px
    }
    @include desktop {
        font-size: 1.0rem; //16px 
        line-height: 1.5rem; // 24px
    }
}

@mixin h3-subheader-white {
    @include h3-subheader;
    color: $white;
}

@mixin h3-subheader-bold {
    @include h3-subheader;
    font-family: AvenirNextLTProBold;
}

@mixin h3-subheader-white-bold {
    @include h3-subheader;
    font-family: AvenirNextLTProBold;
    color: $white;
}


/********************H4 Paragraph & Comments******************************/

@mixin h4-paragraphs-and-comments {
    font-family: AvenirNextLTProRegular;
    color: $black;
    font-size: 0.8rem; // 13px
    line-height: 1.1rem; //18px
    @include tablet {
        font-size: 0.9rem; // 14px
        line-height: 1.4rem; //22px
    }
    @include desktop {
        font-size: 0.9rem; // 14px
        line-height: 1.4rem; // 22px 
    }
}

@mixin h4-paragraphs-and-comments-bold {
    @include h4-paragraphs-and-comments;
    font-family: AvenirNextLTProBold;
}

@mixin h4-paragraphs-and-comments-white {
    @include h4-paragraphs-and-comments;
    color: $white;
}

@mixin h4-paragraphs-and-comments-white-bold {
    @include h4-paragraphs-and-comments-bold;
    color: $white;
}

@mixin h4-paragraphs-and-comments-white-demi {
    @include h4-paragraphs-and-comments-white;
    font-family: AvenirNextLTProDemi;
    color: $white;
}

$time: 330ms;
$btns: ( 
alpha: ( gstart: #21D4FD,
gend: #B721FF,
tstart: #B721FF,
tend: #21D4FD),
beta: ( gstart: #08AEEA,
gend: #2AF598,
tstart: #2AF598,
tend: #08AEEA),
gamma: ( gstart: #FEE140,
gend: #FA709A,
tstart: #FA709A,
tend: #FEE140),
delta: ( gstart: #3EECAC,
gend: #EE74E1,
tstart: #EE74E1,
tend: #3EECAC));
.btn {
	outline: none;;
	margin-right: 30px;
	margin-top: 30px;
	font-family: 'Source Sans Pro', sans-serif;
	font-weight: 900;
	padding: 1.25rem 2rem;
	font-size: 1rem;
	border-radius: 3.5rem / 100%;
	position: relative;
	min-width: 15rem;
	max-width: 90vw;
	overflow: hidden;
	border: 0;
	cursor: pointer;
	text-transform: uppercase;
	letter-spacing: 0.05em;
	transition: all $time;
	outline: none;;
	&+& {
		outline: none;;
		margin-top: 1rem;
	}
	span {
		outline: none;;
		position: relative;
		z-index: 1;
	}
	&:before {
		outline: none;;
		content: "";
		background-color: #21D4FD;
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		transform: scale(4) translateX(-100%);
		transition: all $time * 1.5 ease-out;
		outline: none;;
	}
	&:hover,
	&:focus,
	&:active {
		&:before {
			outline: none;;
			transform: scale(4) translate(37%);
		}
	}
}

@each $bname,
$bcolors in $btns {
	.btn--#{$bname} {
		outline: none;;
		background-color: map-get($bcolors, gstart);
		color: map-get($bcolors, tstart);
		&:before {
			background-color: map-get($bcolors, gend);
			background-image: linear-gradient(to right, map-get($bcolors, gend) 30%, map-get($bcolors, gstart) 100%);
			position: absolute;
		}
		&:hover,
		&:focus,
		&:active {
			outline: none;;
			color: map-get($bcolors, tend);
		}
	}
}

.btn-form {
	outline: none;;
	outline: none;;
	margin-right: 30px;
	margin-top: 30px;
	font-family: 'Source Sans Pro', sans-serif;
	font-weight: 900;
	padding: 1.25rem 2rem;
	font-size: 1rem;
	border-radius: 3.5rem / 100%;
	position: relative;
	min-width: 15rem;
	max-width: 90vw;
	overflow: hidden;
	border: 0;
	cursor: pointer;
	text-transform: uppercase;
	letter-spacing: 0.05em;
	transition: all $time;
	 ;
	&+& {
		margin-top: 1rem;
		 ;
	}
	span {
		position: relative;
		z-index: 1;
		 ;
	}
	&:before {
		content: "";
		background-color: #21D4FD;
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		transform: scale(4) translateX(-100%);
		transition: all $time * 1.5 ease-out;
		 ;
	}
	&:hover,
	&:focus,
	&:active {
		&:before {
			 
			//transform: scale(4) translate(37%);
		  //  color: map-get($bcolors, tstart);
		  background-color:#B721FF;;
		}
	}
}

@each $bname,
$bcolors in $btns {
	.btn-form--#{$bname} {
		background-color: map-get($bcolors, gstart);
		color: map-get($bcolors, tstart);
		&:before {
			background-color: map-get($bcolors, gend);
			background-image: linear-gradient(to right, map-get($bcolors, gend) 30%, map-get($bcolors, gstart) 100%);
			position: absolute;
		}
		&:hover,
		&:focus,
		&:active {
		//	color: map-get($bcolors, tstart);
		color: #B721FF;
		}
	}
}
/********************************h4-lists-and-contacts-information*************/

@mixin h4-list-contact-information {
    color: $black;
    font-family: AvenirNextLTProRegular;
    font-size: 0.8rem; //13px 
    line-height: 1.1rem; //18px 
    @include tablet {
        font-size: 0.9rem; //14px 
        line-height: 1.3rem; //20px
    }
    @include desktop {
        font-size: 0.9rem; //14px 
        line-height: 1.3rem; //20px 
    }
}

@mixin h4-list-contact-information-bold {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProBold;
}

@mixin h4-list-contact-information-demi {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProDemi;
}

@mixin h4-list-contact-information-white {
    @include h4-list-contact-information;
    color: $white;
}

@mixin h4-list-contact-information-white-demi {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProDemi;
    color: $white;
}

@mixin h4-list-contact-information-white-bold {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProBold;
    color: $white;
}


//********************************P-Paragraph****************************/

@mixin p-paragraph {
    font-family: AvenirNextLTProRegular;
    color: $black;
    font-size: 0.8rem; //13px
    line-height: 1.1rem; //18px
    @include tablet {
        font-size: 0.9rem; //13px 
        line-height: 1.3rem; //18px 
        display: inline-block;
    }
    @include desktop {
        font-size: 0.9rem; //13px 
        line-height: 1.3rem; //18px
        display: inline-block;
    }
}

@mixin p-paragraph-bold {
    @include p-paragraph;
    font-family: AvenirNextLTProBold;
}

@mixin p-paragraph-white {
    @include p-paragraph;
    color: white;
}

@mixin p-paragraph-white-bold {
    @include p-paragraph-bold;
    color: white;
}

@mixin h1-page-header {
	font-family: TitilliumWebSemiBold600;
	color: $black;
	font-size: 28px; //28px
	line-height: 36px; //36px
	@include tablet {
        font-size: 32px; //32px
        line-height: 40px; //40px
	}
	@include desktop {
        font-size: 32px; //32px
        line-height: 40px; //40px
	}
}

@mixin h2-subheader {
	font-family: TitilliumWebSemiBold600;
	color: $black;
	font-size: 20px; //20px
	line-height: 28px; //28px
	@include tablet {
        font-size: 24px; //24px
        line-height: 32px; //28px
	}
	@include desktop {
        font-size: 24px; //24px
        line-height: 32px; //28px
	}
}
@mixin h3-labels-links-buttons {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 13px; //13px
	line-height: 20px; //20px
	@include tablet {
        font-size: 14px; //14px
        line-height: 22px; //22px
	}
	@include desktop {
        font-size: 14px; //14px
        line-height: 22px; //22px
	}
}
@mixin h4-labels-table-header {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 11px; //11px
	line-height: 16px; //16px
	@include tablet {
        font-size: 12px; //12px
        line-height: 18px; //18px
	}
	@include desktop {
        font-size: 12px; //12px
        line-height: 18px; //18px
	}
}
@mixin p1-body-large {
	font-family: TitilliumWebRegular400;
	color: $black;
    font-size: 15px; //15px
    line-height: 26px; //26px
	@include tablet {
        font-size: 16px; //12px
        line-height: 28px; //18px
	}
	@include desktop {
        font-size: 16px; //12px
        line-height: 28px; //18px
	}
}
@mixin p2-body-medium {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 16px; //13px
	line-height: 28px; //20px
	@include tablet {
        font-size: 13px; //13px
        line-height: 20px; //20px
	}
	@include desktop {
        font-size: 13px; //13px
        line-height: 20px; //20px
	}
}
@mixin p2-body-small {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 11px; //13px
	line-height: 16px; //20px
	@include tablet {
        font-size: 12px; //13px
        line-height: 18px; //20px
	}
	@include desktop {
        font-size: 12px; //13px
        line-height: 18px; //20px
	}
}

/*******************************************H5-Labels***********************/

@mixin h5-labels {
    font-size: 0.8rem; //12px
    line-height: 1.3rem; //20px 
    font-family: AvenirNextLTProRegular;
    color: $silver; //labels
    @include tablet {
        font-size: 0.8rem; //12px
        line-height: 1.3rem; // 20px
        font-family: AvenirNextLTProDemi;
    }
    @include desktop {
        font-size: 0.8rem; //12px
        line-height: 1.8rem; // 20px
    }
}

@mixin h5-labels-demi {
    @include h5-labels;
    font-family: AvenirNextLTProDemi;
}

@mixin h5-labels-bold {
    @include h5-labels;
    font-family: AvenirNextLTProBold;
}


//Breakpoints
@mixin tablet {
    @media (min-width: $breakpoint-tablet) {
        @content;
    }
}

@mixin desktop {
    @media (min-width: $breakpoint-desktop) {
        @content;
    }
}

//Text Styles- Desktop

@mixin d-header {
    font-family: $font-family;
    font-size: 1.875rem;
    line-height: 2.25rem;
    font-weight: 700;
	color: $text-black;
}

@mixin d-subheader {
	font-family: $font-family;
    font-size: 1.875rem;
    line-height: 2.25rem;
    font-weight: 500;
	color: $text-black;
}

@mixin d-subheader-grey {
	font-family: $font-family;
    font-size: 1.75rem;
    line-height: 2rem;
	color: $text-grey;
    font-weight: 500;
}

@mixin d-paragraph-bold {
	font-family: $font-family;
    font-size: 1.125rem;
    line-height: 1.25rem;
	color: $text-black;
    font-weight: 700;
}

@mixin d-paragraph-large {
	font-family: $font-family;
    font-size: 1.375rem;
    line-height: 1.4rem;
	color: $text-grey;
    font-weight: 400;
}

@mixin d-paragraph-medium {
	font-family: $font-family;
    font-size: 1.25rem;
    line-height: 1.4rem;
	color: $text-grey;
    font-weight: 400;
}

@mixin d-paragraph-small {
	font-family: $font-family;
    font-size: 1.4rem;
    line-height: 1rem;
	color: $text-grey;
    font-weight: 400;
}

//Button Styling
@mixin button-blue {
    background-color: $secondary-color-indigo;
    border-radius: 5px;
    color: white;
    padding: 0.875rem 1.813rem;
    appearance: none;
    border: none;
}

//Component Styling

@mixin component-styling {
    box-shadow: 0px 10px 25px rgba(19, 24, 44, 0.1);
    border-radius: 10px;
    padding: 3.75rem;
}
/***************************************STANDARD PADDING**************************************************/
@mixin standard-padding-right {
	padding-right: $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding-right: $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding-right: $mockup-column-outside-desktop; //80px right 80px left
	}
  }

  @mixin standard-padding-left {
	padding-left: $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding-left: $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding-left: $mockup-column-outside-desktop; //80px right 80px left
	}
  }
  
  @mixin standard-padding-horizontal {
	padding: 0 $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding: 0 $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding: 0 $mockup-column-outside-desktop; //80px right 80px left
	}
  }
  
  
  /**********************************WIDTH*100%*PADDING**********************************************/
  
  @mixin width-100-percent-minus-standard-padding-horizontal {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent} * 2));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent} * 2));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent} * 2));
	}
  }
  
  @mixin width-100-percent-minus-standard-padding-left {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent}));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent}));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent}));
	}
  }
  
  @mixin width-100-percent-minus-standard-padding-right {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent}));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent}));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent}));
	}
  }
  

  /*******************WIDTH MINUS MOCKUP COLUMN************/
  @mixin width-100-percent-minus-mockup-column-outside {
	width: calc(100% - #{$mockup-column-outside-mobile});
	@include tablet {
	  width: calc(100% - #{$mockup-column-outside-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-outside-desktop});
	}
  }
  @mixin width-100-percent-minus-mockup-column-outside-horizontal {
	width: calc(100% - (#{$mockup-column-outside-mobile} * 2));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet} *2));
	}
	@include desktop {
	  width: calc(100% - (#{mockup-column-outside-desktop} *2));
	}
  }
  @mixin width-100-percent-minus-mockup-column-small {
	width: calc(100% - #{mockup-column-small-mobile});
	@include tablet {
	  width: calc(100% -#{mockup-column-small-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-small-desktop});
	}
  }
  
  @mixin width-100-percent-minus-mockup-column-large {
	width: calc(100% - #{mockup-column-large-mobile});
	@include tablet {
	  width: calc(100% -#{mockup-column-large-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-large-desktop});
	}
  }
  



@mixin fancybutton {
  font-family: 'Source Sans Pro', sans-serif;
  font-weight: 900;
  padding: 1.25rem 2rem;
  font-size: 1rem;
  border-radius: 3.5rem / 100%;
  position: relative;
  min-width: 15rem;
  max-width: 90vw;
  overflow: hidden;
  border: 0;
  cursor: pointer;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  transition: all $time;
  
  & + & {
    margin-top: 1rem;
  }
  
  span {
    position: relative;
    z-index: 1;
  }
  
  &:before {
    content: "";
    background-color: #21D4FD;
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    transform: scale(4) translateX(-100%);
    transition: all $time * 1.5 ease-out;
  }
  
  &:hover,
  &:focus,
  &:active {    
    &:before {
      transform: scale(4) translate(37%);
    } 
  }
}


  /**********************************************DIVIDERS******************************************************/
  
  @mixin divider {
	background: $silver;
	width: 100%;
	height: 1px;
  }
  
  @mixin divider-form {
	background: $silver;
	width: 100%;
	height: 1px;
	margin-bottom: $increment2-px;
  }
  
  /**********************************************LOGOS******************************************************/
  
  @mixin logo-brainflix {
	img {
	  width: 120px; //120px
	  height: 24px; //24px
	  margin-left: 1rem;
	  @include tablet {
		margin-left: 2rem;
		width: 120px; //120px
		height: 24px; //24px
	  }
	  @include desktop {
		margin-left: 5rem;
	  }
	}
  }
  
  /**********************************************BUTTONS******************************************************/
  
  @mixin button {
	background-color: $brainflix-blue;
	@include h4-paragraphs-and-comments-white-demi;
	&:hover {
	  background: $brainflix-dark-blue-hover;
	}
	&:focus {
	  outline: none;
	}
	width: $increment28-px; //226px
	height: $increment6-px; 
	border-radius: $increment0-px; 
	border-color: $brainflix-blue;
	@include tablet {
	  padding: 0;
	  width: $increment20-px; //160px
	  height: $increment6-px; 
	}
  }
  
  /**********************************************MOCKUP COLUMNS****************************************************************/
  
  /*******************RIGHT*****************/
  
  @mixin mockup-column-outside-right {
	margin-right: $mockup-column-outside-mobile;
	@include tablet {
	  margin-right: $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-right {
	margin-right: $mockup-column-small-mobile;
	@include tablet {
	  margin-right: $mockup-column-small-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-right {
	margin-right: $mockup-column-large-mobile;
	@include tablet {
	  margin-right: $mockup-column-large-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-large-desktop;
	}
  }
  
  /*******************LEFT***************/
  
  @mixin mockup-column-outside-left {
	margin-left: $mockup-column-outside-mobile;
	@include tablet {
	  margin-left: $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-left {
	margin-left: $mockup-column-small-mobile;
	@include tablet {
	  margin-left: $mockup-column-small-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-left {
	margin-left: $mockup-column-large-mobile;
	@include tablet {
	  margin-left: $mockup-column-large-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-large-desktop;
	}
  }
  
  /*********************HORIZONTAL**************/
  
  @mixin mockup-column-outside-horizontal {
	margin: 0 $mockup-column-outside-mobile;
	@include tablet {
	  margin: 0 $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-horizontal {
	margin: 0 $mockup-column-small-mobile;
	@include tablet {
	  margin: 0 $mockup-column-small-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-horizontal {
	margin: 0 $mockup-column-large-mobile;
	@include tablet {
	  margin: 0 $mockup-column-large-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-large-desktop;
	}
  }
  
  /*********************************************WIDTH 100% MINUS INCREMENTS*********************************/
  
  @mixin width-100-percent-minus-increment0 {
	width: calc(100% - #{increment0-mobile});
	@include tablet {
	  width: calc(100% -#{increment0-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment0-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment1 {
	width: calc(100% - #{increment1-mobile});
	@include tablet {
	  width: calc(100% - #{increment1-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment1-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment2 {
	width: calc(100% - #{increment2-mobile});
	@include tablet {
	  width: calc(100% -#{increment2-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment2-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment3 {
	width: calc(100% - #{increment3-mobile});
	@include tablet {
	  width: calc(100% -#{increment3-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment3-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment4 {
	width: calc(100% - #{increment4-mobile});
	@include tablet {
	  width: calc(100% -#{increment4-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment4-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment5 {
	width: calc(100% - #{increment5-mobile});
	@include tablet {
	  width: calc(100% -#{increment5-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment5-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment6 {
	width: calc(100% - #{increment6-mobile});
	@include tablet {
	  width: calc(100% -#{increment6-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment6-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment7 {
	width: calc(100% - #{increment7-mobile});
	@include tablet {
	  width: calc(100% -#{increment7-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment7-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment8 {
	width: calc(100% - #{increment8-mobile});
	@include tablet {
	  width: calc(100% -#{increment8-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment8-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment9 {
	width: calc(100% - #{increment9-mobile});
	@include tablet {
	  width: calc(100% -#{increment9-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment9-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment10 {
	width: calc(100% - #{increment10-mobile});
	@include tablet {
	  width: calc(100% -#{increment10-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment10-desktop});
	}
  }
  

  
  /************************************%%%%%%%%%%%%%%%%%%%%%******************************************************
  /******%%%%%%%%%%%%%%%%%%%%%%%****CONTENTS LIST:*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****TOP MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *****%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%******RIGHT MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***BOTTOM MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***LEFT MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ***********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*VERTICAL MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ***********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*HORIZTONAL**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***WIDTH **********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%****HEIGHT**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****OTHER COMMON INCREMENTS*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
   ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****MOCKUP COLUMN PADDING*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********/
  
  /*******************************************************INCREMENT MARGIN TOP******************************************/
  
  @mixin increment-0-top {
	margin-top: $increment0-mobile;
	@include tablet {
	  margin-top: $increment0-tablet;
	}
	@include desktop {
	  margin-top: $increment0-desktop;
	}
  }
  
  @mixin increment-1-top {
	margin-top: $increment1-mobile;
	@include tablet {
	  margin-top: $increment1-tablet;
	}
	@include desktop {
	  margin-top: $increment1-desktop;
	}
  }
  
  @mixin increment-2-top {
	margin-top: $increment2-mobile;
	@include tablet {
	  margin-top: $increment2-tablet;
	}
	@include desktop {
	  margin-top: $increment2-desktop;
	}
  }
  
  @mixin increment-3-top {
	margin-top: $increment3-mobile;
	@include tablet {
	  margin-top: $increment3-tablet;
	}
	@include desktop {
	  margin-top: $increment3-desktop;
	}
  }
  
  @mixin increment-4-top {
	margin-top: $increment4-mobile;
	@include tablet {
	  margin-top: $increment4-tablet;
	}
	@include desktop {
	  margin-top: $increment4-desktop;
	}
  }
  
  @mixin increment-5-top {
	margin-top: $increment5-mobile;
	@include tablet {
	  margin-top: $increment5-tablet;
	}
	@include desktop {
	  margin-top: $increment5-desktop;
	}
  }
  
  @mixin increment-6-top {
	margin-top: $increment6-mobile;
	@include tablet {
	  margin-top: $increment6-tablet;
	}
	@include desktop {
	  margin-top: $increment6-desktop;
	}
  }
  
  @mixin increment-7-top {
	margin-top: $increment7-mobile;
	@include tablet {
	  margin-top: $increment7-tablet;
	}
	@include desktop {
	  margin-top: $increment7-desktop;
	}
  }
  
  @mixin increment-8-top {
	margin-top: $increment8-mobile;
	@include tablet {
	  margin-top: $increment8-tablet;
	}
	@include desktop {
	  margin-top: $increment8--desktop;
	}
  }
  
  @mixin increment-9-top {
	margin-top: $increment9-mobile;
	@include tablet {
	  margin-top: $increment9-tablet;
	}
	@include desktop {
	  margin-top: $increment9--desktop;
	}
  }
  
  @mixin increment-10-top {
	margin-top: $increment10-mobile;
	@include tablet {
	  margin-top: $increment10-tablet;
	}
	@include desktop {
	  margin-top: $increment10--desktop;
	}
  }
  
  /***********************************INCREMENT RIGHT*************************************************/
  
  @mixin increment-0-right {
	margin-right: $increment0-mobile;
	@include tablet {
	  margin-right: $increment0-tablet;
	}
	@include desktop {
	  margin-right: $increment0-desktop;
	}
  }
  
  @mixin increment-1-right {
	margin-right: $increment1-mobile;
	@include tablet {
	  margin-right: $increment1-tablet;
	}
	@include desktop {
	  margin-right: $increment1-desktop;
	}
  }
  
  @mixin increment-2-right {
	margin-right: $increment2-mobile;
	@include tablet {
	  margin-right: $increment2-tablet;
	}
	@include desktop {
	  margin-right: $increment2-desktop;
	}
  }
  
  @mixin increment-3-right {
	margin-right: $increment3-mobile;
	@include tablet {
	  margin-right: $increment3-tablet;
	}
	@include desktop {
	  margin-right: $increment3-desktop;
	}
  }
  
  @mixin increment-4-right {
	margin-right: $increment4-mobile;
	@include tablet {
	  margin-right: $increment4-tablet;
	}
	@include desktop {
	  margin-right: $increment4-desktop;
	}
  }
  
  @mixin increment-5-right {
	margin-right: $increment5-mobile;
	@include tablet {
	  margin-right: $increment5-tablet;
	}
	@include desktop {
	  margin-right: $increment5-desktop;
	}
  }
  
  @mixin increment-6-right {
	margin-right: $increment6-mobile;
	@include tablet {
	  margin-right: $increment6-tablet;
	}
	@include desktop {
	  margin-right: $increment6-desktop;
	}
  }
  
  @mixin increment-7-right {
	margin-right: $increment7-mobile;
	@include tablet {
	  margin-right: $increment7-tablet;
	}
	@include desktop {
	  margin-right: $increment7-desktop;
	}
  }
  
  @mixin increment-8-right {
	margin-right: $increment8-mobile;
	@include tablet {
	  margin-right: $increment8-tablet;
	}
	@include desktop {
	  margin-right: $increment8--desktop;
	}
  }
  
  @mixin increment-9-right {
	margin-right: $increment9-mobile;
	@include tablet {
	  margin-right: $increment9-tablet;
	}
	@include desktop {
	  margin-right: $increment9--desktop;
	}
  }
  
  @mixin increment-10-right {
	margin-right: $increment10-mobile;
	@include tablet {
	  margin-right: $increment10-tablet;
	}
	@include desktop {
	  margin-right: $increment10--desktop;
	}
  }
  
  /*******************************************************INCREMENT MARGIN BOTTOM******************************************/
  
  @mixin increment-0-bottom {
	margin-bottom: $increment0-mobile;
	@include tablet {
	  margin-bottom: $increment0-tablet;
	}
	@include desktop {
	  margin-bottom: $increment0-desktop;
	}
  }
  
  @mixin increment-1-bottom {
	margin-bottom: $increment1-mobile;
	@include tablet {
	  margin-bottom: $increment1-tablet;
	}
	@include desktop {
	  margin-bottom: $increment1-desktop;
	}
  }
  
  @mixin increment-2-bottom {
	margin-bottom: $increment2-mobile;
	@include tablet {
	  margin-bottom: $increment2-tablet;
	}
	@include desktop {
	  margin-bottom: $increment2-desktop;
	}
  }
  
  @mixin increment-3-bottom {
	margin-bottom: $increment3-mobile;
	@include tablet {
	  margin-bottom: $increment3-tablet;
	}
	@include desktop {
	  margin-bottom: $increment3-desktop;
	}
  }
  
  @mixin increment-4-bottom {
	margin-bottom: $increment4-mobile;
	@include tablet {
	  margin-bottom: $increment4-tablet;
	}
	@include desktop {
	  margin-bottom: $increment4-desktop;
	}
  }
  
  @mixin increment-5-bottom {
	margin-bottom: $increment5-mobile;
	@include tablet {
	  margin-bottom: $increment5-tablet;
	}
	@include desktop {
	  margin-bottom: $increment5-desktop;
	}
  }
  
  @mixin increment-6-bottom {
	margin-bottom: $increment6-mobile;
	@include tablet {
	  margin-bottom: $increment6-tablet;
	}
	@include desktop {
	  margin-bottom: $increment6-desktop;
	}
  }
  
  @mixin increment-7-bottom {
	margin-bottom: $increment7-mobile;
	@include tablet {
	  margin-bottom: $increment7-tablet;
	}
	@include desktop {
	  margin-bottom: $increment7-desktop;
	}
  }
  
  @mixin increment-8-bottom {
	margin-bottom: $increment8-mobile;
	@include tablet {
	  margin-bottom: $increment8-tablet;
	}
	@include desktop {
	  margin-bottom: $increment8--desktop;
	}
  }
  
  @mixin increment-9-bottom {
	margin-bottom: $increment9-mobile;
	@include tablet {
	  margin-bottom: $increment9-tablet;
	}
	@include desktop {
	  margin-bottom: $increment9--desktop;
	}
  }
  
  @mixin increment-10-bottom {
	margin-bottom: $increment10-mobile;
	@include tablet {
	  margin-bottom: $increment10-tablet;
	}
	@include desktop {
	  margin-bottom: $increment10--desktop;
	}
  }
  
  /*******************************************************INCREMENT LEFT******************************************/
  
  @mixin increment-0-left {
	margin-left: $increment0-mobile;
	@include tablet {
	  margin-left: $increment0-tablet;
	}
	@include desktop {
	  margin-left: $increment0-desktop;
	}
  }
  
  @mixin increment-1-left {
	margin-left: $increment1-mobile;
	@include tablet {
	  margin-left: $increment1-tablet;
	}
	@include desktop {
	  margin-left: $increment1-desktop;
	}
  }
  
  @mixin increment-2-left {
	margin-left: $increment2-mobile;
	@include tablet {
	  margin-left: $increment2-tablet;
	}
	@include desktop {
	  margin-left: $increment2-desktop;
	}
  }
  
  @mixin increment-3-left {
	margin-left: $increment3-mobile;
	@include tablet {
	  margin-left: $increment3-tablet;
	}
	@include desktop {
	  margin-left: $increment3-desktop;
	}
  }
  
  @mixin increment-4-left {
	margin-left: $increment4-mobile;
	@include tablet {
	  margin-left: $increment4-tablet;
	}
	@include desktop {
	  margin-left: $increment4-desktop;
	}
  }
  
  @mixin increment-5-left {
	margin-left: $increment5-mobile;
	@include tablet {
	  margin-left: $increment5-tablet;
	}
	@include desktop {
	  margin-left: $increment5-desktop;
	}
  }
  
  @mixin increment-6-left {
	margin-left: $increment6-mobile;
	@include tablet {
	  margin-left: $increment6-tablet;
	}
	@include desktop {
	  margin-left: $increment6-desktop;
	}
  }
  
  @mixin increment-7-left {
	margin-left: $increment7-mobile;
	@include tablet {
	  margin-left: $increment7-tablet;
	}
	@include desktop {
	  margin-left: $increment7-desktop;
	}
  }
  
  @mixin increment-8-left {
	margin-left: $increment8-mobile;
	@include tablet {
	  margin-left: $increment8-tablet;
	}
	@include desktop {
	  margin-left: $increment8--desktop;
	}
  }
  
  @mixin increment-9-left {
	margin-left: $increment9-mobile;
	@include tablet {
	  margin-left: $increment9-tablet;
	}
	@include desktop {
	  margin-left: $increment9--desktop;
	}
  }
  
  @mixin increment-10-left {
	margin-left: $increment10-mobile;
	@include tablet {
	  margin-left: $increment10-tablet;
	}
	@include desktop {
	  margin-left: $increment10--desktop;
	}
  }
  
  /************************************VERTICAL MARGIN INCREMENTS******************************************************/
  
  @mixin increment-0-vertical {
	margin: $increment0-mobile 0;
	@include tablet {
	  margin: $increment0-tablet 0;
	}
	@include deskvertical {
	  margin: $increment0-desktop 0;
	}
  }
  
  @mixin increment-1-vertical {
	margin: $increment1-mobile 0;
	@include tablet {
	  margin: $increment1-tablet 0;
	}
	@include desktop {
	  margin: $increment1-desktop 0;
	}
  }
  
  @mixin increment-2-vertical {
	margin: $increment2-mobile 0;
	@include tablet {
	  margin: $increment2-tablet 0;
	}
	@include desktop {
	  margin: $increment2-desktop 0;
	}
  }
  
  @mixin increment-3-vertical {
	margin: $increment3-mobile 0;
	@include tablet {
	  margin: $increment3-tablet 0;
	}
	@include desktop {
	  margin: $increment3-desktop 0;
	}
  }
  
  @mixin increment-4-vertical {
	margin: $increment4-mobile 0;
	@include tablet {
	  margin: $increment4-tablet 0;
	}
	@include desktop {
	  margin: $increment4-desktop 0;
	}
  }
  
  @mixin increment-5-vertical {
	margin: $increment5-mobile 0;
	@include tablet {
	  margin: $increment5-tablet 0;
	}
	@include desktop {
	  margin: $increment5-desktop 0;
	}
  }
  
  @mixin increment-6-vertical {
	margin: $increment6-mobile 0;
	@include tablet {
	  margin: $increment6-tablet 0;
	}
	@include desktop {
	  margin: $increment6-desktop 0;
	}
  }
  
  @mixin increment-7-vertical {
	margin: $increment7-mobile 0;
	@include tablet {
	  margin: $increment7-tablet 0;
	}
	@include desktop {
	  margin: $increment7-desktop 0;
	}
  }
  
  @mixin increment-8-vertical {
	margin: $increment8-mobile 0;
	@include tablet {
	  margin: $increment8-tablet 0;
	}
	@include desktop {
	  margin: $increment8--desktop 0;
	}
  }
  
  @mixin increment-9-vertical {
	margin: $increment9-mobile 0;
	@include tablet {
	  margin: $increment9-tablet 0;
	}
	@include desktop {
	  margin: $increment9--desktop 0;
	}
  }
  
  @mixin increment-10-vertical {
	margin: $increment10-mobile 0;
	@include tablet {
	  margin: $increment10-tablet 0;
	}
	@include desktop {
	  margin: $increment10--desktop 0;
	}
  }
  
  /************************************HORIZONTAL MARGIN INCREMENTS******************************************************/
  
  @mixin increment-0-horizontal {
	margin: 0 $increment0-mobile;
	@include tablet {
	  margin: 0 $increment0-tablet;
	}
	@include deskhorizontal {
	  margin: 0 $increment0-desktop;
	}
  }
  
  @mixin increment-1-horizontal {
	margin: 0 $increment1-mobile;
	@include tablet {
	  margin: 0 $increment1-tablet;
	}
	@include desktop {
	  margin: 0 $increment1-desktop;
	}
  }
  
  @mixin increment-2-horizontal {
	margin: 0 $increment2-mobile;
	@include tablet {
	  margin: 0 $increment2-tablet;
	}
	@include desktop {
	  margin: 0 $increment2-desktop;
	}
  }
  
  @mixin increment-3-horizontal {
	margin: 0 $increment3-mobile;
	@include tablet {
	  margin: 0 $increment3-tablet;
	}
	@include desktop {
	  margin: 0 $increment3-desktop;
	}
  }
  
  @mixin increment-4-horizontal {
	margin: 0 $increment4-mobile;
	@include tablet {
	  margin: 0 $increment4-tablet;
	}
	@include desktop {
	  margin: 0 $increment4-desktop;
	}
  }
  
  @mixin increment-5-horizontal {
	margin: 0 $increment5-mobile;
	@include tablet {
	  margin: 0 $increment5-tablet;
	}
	@include desktop {
	  margin: 0 $increment5-desktop;
	}
  }
  
  @mixin increment-6-horizontal {
	margin: 0 $increment6-mobile;
	@include tablet {
	  margin: 0 $increment6-tablet;
	}
	@include desktop {
	  margin: 0 $increment6-desktop;
	}
  }
  
  @mixin increment-7-horizontal {
	margin: 0 $increment7-mobile;
	@include tablet {
	  margin: 0 $increment7-tablet;
	}
	@include desktop {
	  margin: 0 $increment7-desktop;
	}
  }
  
  @mixin increment-8-horizontal {
	margin: 0 $increment8-mobile;
	@include tablet {
	  margin: 0 $increment8-tablet;
	}
	@include desktop {
	  margin: 0 $increment8--desktop;
	}
  }
  
  @mixin increment-9-horizontal {
	margin: 0 $increment9-mobile;
	@include tablet {
	  margin: 0 $increment9-tablet;
	}
	@include desktop {
	  margin: 0 $increment9--desktop;
	}
  }
  
  @mixin increment-10-horizontal {
	margin: 0 $increment10-mobile;
	@include tablet {
	  margin: 0 $increment10-tablet;
	}
	@include desktop {
	  margin: 0 $increment10--desktop;
	}
  }
    /***************************************************INCREMENT WIDTH******************************************/
  
	@mixin increment-0-width {
		width: $increment0-mobile;
		@include tablet {
		  width: $increment0-tablet;
		}
		@include deskwidth {
		  width: $increment0-desktop;
		}
	  }
	  
	  @mixin increment-1-width {
		width: $increment1-mobile;
		@include tablet {
		  width: $increment1-tablet;
		}
		@include desktop {
		  width: $increment1-desktop;
		}
	  }
	  
	  @mixin increment-2-width {
		width: $increment2-mobile;
		@include tablet {
		  width: $increment2-tablet;
		}
		@include desktop {
		  width: $increment2-desktop;
		}
	  }
	  
	  @mixin increment-3-width {
		width: $increment3-mobile;
		@include tablet {
		  width: $increment3-tablet;
		}
		@include desktop {
		  width: $increment3-desktop;
		}
	  }
	  
	  @mixin increment-4-width {
		width: $increment4-mobile;
		@include tablet {
		  width: $increment4-tablet;
		}
		@include desktop {
		  width: $increment4-desktop;
		}
	  }
	  
	  @mixin increment-5-width {
		width: $increment5-mobile;
		@include tablet {
		  width: $increment5-tablet;
		}
		@include desktop {
		  width: $increment5-desktop;
		}
	  }
	  
	  @mixin increment-6-width {
		width: $increment6-mobile;
		@include tablet {
		  width: $increment6-tablet;
		}
		@include desktop {
		  width: $increment6-desktop;
		}
	  }
	  
	  @mixin increment-7-width {
		width: $increment7-mobile;
		@include tablet {
		  width: $increment7-tablet;
		}
		@include desktop {
		  width: $increment7-desktop;
		}
	  }
	  
	  @mixin increment-8-width {
		width: $increment8-mobile;
		@include tablet {
		  width: $increment8-tablet;
		}
		@include desktop {
		  width: $increment8--desktop;
		}
	  }
	  
	  @mixin increment-9-width {
		width: $increment9-mobile;
		@include tablet {
		  width: $increment9-tablet;
		}
		@include desktop {
		  width: $increment9--desktop;
		}
	  }
	  
	  @mixin increment-10-width {
		width: $increment10-mobile;
		@include tablet {
		  width: $increment10-tablet;
		}
		@include desktop {
		  width: $increment10--desktop;
		}
	  }
  
  /***************************************************INCREMENT HEIGHT******************************************/
  
  @mixin increment-0-height {
	height: $increment0-mobile;
	@include tablet {
	  height: $increment0-tablet;
	}
	@include deskheight {
	  height: $increment0-desktop;
	}
  }
  
  @mixin increment-1-height {
	height: $increment1-mobile;
	@include tablet {
	  height: $increment1-tablet;
	}
	@include desktop {
	  height: $increment1-desktop;
	}
  }
  
  @mixin increment-2-height {
	height: $increment2-mobile;
	@include tablet {
	  height: $increment2-tablet;
	}
	@include desktop {
	  height: $increment2-desktop;
	}
  }
  
  @mixin increment-3-height {
	height: $increment3-mobile;
	@include tablet {
	  height: $increment3-tablet;
	}
	@include desktop {
	  height: $increment3-desktop;
	}
  }
  
  @mixin increment-4-height {
	height: $increment4-mobile;
	@include tablet {
	  height: $increment4-tablet;
	}
	@include desktop {
	  height: $increment4-desktop;
	}
  }
  
  @mixin increment-5-height {
	height: $increment5-mobile;
	@include tablet {
	  height: $increment5-tablet;
	}
	@include desktop {
	  height: $increment5-desktop;
	}
  }
  
  @mixin increment-6-height {
	height: $increment6-mobile;
	@include tablet {
	  height: $increment6-tablet;
	}
	@include desktop {
	  height: $increment6-desktop;
	}
  }
  
  @mixin increment-7-height {
	height: $increment7-mobile;
	@include tablet {
	  height: $increment7-tablet;
	}
	@include desktop {
	  height: $increment7-desktop;
	}
  }
  
  @mixin increment-8-height {
	height: $increment8-mobile;
	@include tablet {
	  height: $increment8-tablet;
	}
	@include desktop {
	  height: $increment8--desktop;
	}
  }
  
  @mixin increment-9-height {
	height: $increment9-mobile;
	@include tablet {
	  height: $increment9-tablet;
	}
	@include desktop {
	  height: $increment9--desktop;
	}
  }
  
  @mixin increment-10-height {
	height: $increment10-mobile;
	@include tablet {
	  height: $increment10-tablet;
	}
	@include desktop {
	  height: $increment10--desktop;
	}
  }
  
  /***************************************************OTHER COMMON INCREMENTS********************************/
  
  /*******TOP****/
  
  @mixin increment20px-top {
	margin-top: $increment20px-mobile;
	@include tablet {
	  margin-top: $increment20px-tablet;
	}
	@include desktop {
	  margin-top: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-top {
	margin-top: $increment54px-mobile;
	@include tablet {
	  margin-top: $increment54px-tablet;
	}
	@include desktop {
	  margin-top: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-top {
	margin-top: $increment68px-mobile;
	@include tablet {
	  margin-top: $increment68px-tablet;
	}
	@include desktop {
	  margin-top: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-top {
	margin-top: $increment100px-mobile;
	@include tablet {
	  margin-top: $increment100px-tablet;
	}
	@include desktop {
	  margin-top: $increment100px-desktop;
	}
  }
  
  /***************RIGHT********************/
  
  @mixin increment20px-right {
	margin-right: $increment20px-mobile;
	@include tablet {
	  margin-right: $increment20px-tablet;
	}
	@include desktop {
	  margin-right: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-right {
	margin-right: $increment54px-mobile;
	@include tablet {
	  margin-right: $increment54px-tablet;
	}
	@include desktop {
	  margin-right: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-right {
	margin-right: $increment68px-mobile;
	@include tablet {
	  margin-right: $increment68px-tablet;
	}
	@include desktop {
	  margin-right: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-right {
	margin-right: $increment100px-mobile;
	@include tablet {
	  margin-right: $increment100px-tablet;
	}
	@include desktop {
	  margin-right: $increment100px-desktop;
	}
  }
  
  /****************BOTTOM***************/
  
  @mixin increment20px-bottom {
	margin-bottom: $increment20px-mobile;
	@include tablet {
	  margin-bottom: $increment20px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-bottom {
	margin-bottom: $increment54px-mobile;
	@include tablet {
	  margin-bottom: $increment54px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-bottom {
	margin-bottom: $increment68px-mobile;
	@include tablet {
	  margin-bottom: $increment68px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-bottom {
	margin-bottom: $increment100px-mobile;
	@include tablet {
	  margin-bottom: $increment100px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment100px-desktop;
	}
  }
  
  /*****************LEFT****************/
  
  @mixin increment20px-left {
	margin-left: $increment20px-mobile;
	@include tablet {
	  margin-left: $increment20px-tablet;
	}
	@include desktop {
	  margin-left: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-left {
	margin-left: $increment54px-mobile;
	@include tablet {
	  margin-left: $increment54px-tablet;
	}
	@include desktop {
	  margin-left: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-left {
	margin-left: $increment68px-mobile;
	@include tablet {
	  margin-left: $increment68px-tablet;
	}
	@include desktop {
	  margin-left: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-left {
	margin-left: $increment100px-mobile;
	@include tablet {
	  margin-left: $increment100px-tablet;
	}
	@include desktop {
	  margin-left: $increment100px-desktop;
	}
  }
  
  /************VERTICAL***********/
  
  @mixin increment20px-vertical {
	margin: $increment20px-mobile 0;
	@include tablet {
	  margin: $increment20px-tablet 0;
	}
	@include desktop {
	  margin: $increment20px-desktop 0;
	}
  }
  
  @mixin increment54px-vertical {
	margin: $increment54px-mobile 0;
	@include tablet {
	  margin: $increment54px-tablet 0;
	}
	@include desktop {
	  margin: $increment54px-desktop 0;
	}
  }
  
  @mixin increment68px-vertical {
	margin: $increment68px-mobile 0;
	@include tablet {
	  margin: $increment68px-tablet 0;
	}
	@include desktop {
	  margin: $increment68px-desktop 0;
	}
  }
  
  @mixin increment100px-vertical {
	margin: $increment100px-mobile 0;
	@include tablet {
	  margin: $increment100px-tablet 0;
	}
	@include desktop {
	  margin: $increment100px-desktop 0;
	}
  }
  
  /**********HORIZONTAL*********/
  
  @mixin increment20px-horizontal {
	margin: 0 $increment20px-mobile;
	@include tablet {
	  margin: 0 $increment20px-tablet;
	}
	@include desktop {
	  margin: 0 $increment20px-desktop;
	}
  }
  
  @mixin increment54px-horizontal {
	margin: 0 $increment54px-mobile;
	@include tablet {
	  margin: 0 $increment54px-tablet;
	}
	@include desktop {
	  margin: 0 $increment54px-desktop;
	}
  }
  
  @mixin increment68px-horizontal {
	margin: 0 $increment68px-mobile;
	@include tablet {
	  margin: 0 $increment68px-tablet;
	}
	@include desktop {
	  margin: 0 $increment68px-desktop;
	}
  }
  
  @mixin increment100px-horizontal {
	margin: 0 $increment100px-mobile;
	@include tablet {
	  margin: 0 $increment100px-tablet;
	}
	@include desktop {
	  margin: 0 $increment100px-desktop;
	}
  }
  
 /***********WIDTH*********/
  
  @mixin increment20px-width {
	width: $increment20px-mobile;
	@include tablet {
	  width: $increment20px-tablet;
	}
	@include desktop {
	  width: $increment20px-desktop;
	}
  }

 @mixin increment54px-width {
	width: $increment54px-mobile;
	@include tablet {
	  width: $increment54px-tablet;
	}
	@include desktop {
	  width: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-width {
	width: $increment68px-mobile;
	@include tablet {
	  width: $increment68px-tablet;
	}
	@include desktop {
	  width: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-width {
	width: $increment100px-mobile;
	@include tablet {
	  width: $increment100px-tablet;
	}
	@include desktop {
	  width: $increment100px-desktop;
	}
  }

  /***********HEIGHT*********/
  @mixin increment20px-height {
	height: $increment20px-mobile;
	@include tablet {
	  height: $increment20px-tablet;
	}
	@include desktop {
	  height: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-height {
	height: $increment54px-mobile;
	@include tablet {
	  height: $increment54px-tablet;
	}
	@include desktop {
	  height: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-height {
	height: $increment68px-mobile;
	@include tablet {
	  height: $increment68px-tablet;
	}
	@include desktop {
	  height: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-height {
	height: $increment100px-mobile;
	@include tablet {
	  height: $increment100px-tablet;
	}
	@include desktop {
	  height: $increment100px-desktop;
	}
  }
  
  /**********************************************MOCKUP COLUMNS PADDING****************************************************************/
  
  @mixin mockup-column-padding-outside-right {
	padding-right: $mockup-column-outside-mobile;
	@include tablet {
	  padding-right: $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-right {
	padding-right: $mockup-column-small-mobile;
	@include tablet {
	  padding-right: $mockup-column-small-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-right {
	padding-right: $mockup-column-large-mobile;
	@include tablet {
	  padding-right: $mockup-column-large-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-large-desktop;
	}
  }
  
  /***LEFT*******/
  
  @mixin mockup-column-padding-outside-left {
	padding-left: $mockup-column-outside-mobile;
	@include tablet {
	  padding-left: $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-left {
	padding-left: $mockup-column-small-mobile;
	@include tablet {
	  padding-left: $mockup-column-small-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-left {
	padding-left: $mockup-column-large-mobile;
	@include tablet {
	  padding-left: $mockup-column-large-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-large-desktop;
	}
  }
  
  /***HORIZONTAL*******/
  
  @mixin mockup-column-padding-outside-horizontal {
	padding: 0 $mockup-column-outside-mobile;
	@include tablet {
	  padding: 0 $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-horizontal {
	padding: 0 $mockup-column-small-mobile;
	@include tablet {
	  padding: 0 $mockup-column-small-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-horizontal {
	padding: 0 $mockup-column-large-mobile;
	@include tablet {
	  padding: 0 $mockup-column-large-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-large-desktop;
	}
  }
  /*************************************TEXT STYLES************************************/
@use "./colors" as *;
@use "./breakpoints" as *;
@use "./variables" as *;
/*****************************************H1-Main-heading***************************/

@mixin h1-main-header {
    font-family: AvenirNextLTProBold;
    font-size: 1.5rem; //24px 
    line-height: 2.0rem; //32px
    color: $white;
    @include tablet {
        font-size: 2.3rem; //36px 2.25rem 
        line-height: 2.8rem; //44px
    }
    @include desktop {
        font-size: 2.3rem; //36px 2.25rem 
        line-height: 2.8rem; //44px
    }
}

@mixin h1-main-header-regular {
    @include h1-main-header;
    font-family: AvenirNextLTProRegular;
}

@mixin h1-main-header-black {
    @include h1-main-header;
    color: $black;
}

@mixin h1-desktop-size-that-should-be-available-on-mobile {
    color: $white;
    font-family: AvenirNextLTProBold;
    font-size: 8.0vw;
    line-height: 10.8vw;
}


/*********************H2 - Header subheader for mobile*********************/

@mixin h2-header {
    font-size: 0.9rem; //14px 
    line-height: 1.4rem; //22px 
    font-family: AvenirNextLTProDemi;
    color: $black;
    @include tablet {
        font-size: 1.5rem; //24px 1.5rem 
        line-height: 2.0; //32px 2rem
        font-family: AvenirNextLTProBold;
    }
    @include desktop {
        font-size: 1.5rem; //24px 1.5rem 
        line-height: 2.0rem; //32px 2rem 
    }
}

@mixin h2-header-white {
    @include h2-header;
    color: $white;
}

@mixin h2-header-responsive-white {
    @include h2-header-responsive;
    color: $white;
}

@mixin h2-desktop-size-that-should-be-available-on-mobile {
    color: $white;
    font-size: 1.5rem; //24px 1.5rem 
    line-height: 2.0; //32px 2rem
    font-family: AvenirNextLTProBold;
}


/*******************************H3 - Subheader for Tablet and Desktop************/

@mixin h3-subheader {
    font-size: 1.0rem; //16px 
    line-height: 1.5rem; // 24px
    font-family: AvenirNextLTProDemi;
    color: $black;
    @include tablet {
        font-size: 1.0rem; //16px 
        line-height: 1.5rem; // 24px
    }
    @include desktop {
        font-size: 1.0rem; //16px 
        line-height: 1.5rem; // 24px
    }
}

@mixin h3-subheader-white {
    @include h3-subheader;
    color: $white;
}

@mixin h3-subheader-bold {
    @include h3-subheader;
    font-family: AvenirNextLTProBold;
}

@mixin h3-subheader-white-bold {
    @include h3-subheader;
    font-family: AvenirNextLTProBold;
    color: $white;
}


/********************H4 Paragraph & Comments******************************/

@mixin h4-paragraphs-and-comments {
    font-family: AvenirNextLTProRegular;
    color: $black;
    font-size: 0.8rem; // 13px
    line-height: 1.1rem; //18px
    @include tablet {
        font-size: 0.9rem; // 14px
        line-height: 1.4rem; //22px
    }
    @include desktop {
        font-size: 0.9rem; // 14px
        line-height: 1.4rem; // 22px 
    }
}

@mixin h4-paragraphs-and-comments-bold {
    @include h4-paragraphs-and-comments;
    font-family: AvenirNextLTProBold;
}

@mixin h4-paragraphs-and-comments-white {
    @include h4-paragraphs-and-comments;
    color: $white;
}

@mixin h4-paragraphs-and-comments-white-bold {
    @include h4-paragraphs-and-comments-bold;
    color: $white;
}

@mixin h4-paragraphs-and-comments-white-demi {
    @include h4-paragraphs-and-comments-white;
    font-family: AvenirNextLTProDemi;
    color: $white;
}

$time: 330ms;
$btns: ( 
alpha: ( gstart: #21D4FD,
gend: #B721FF,
tstart: #B721FF,
tend: #21D4FD),
beta: ( gstart: #08AEEA,
gend: #2AF598,
tstart: #2AF598,
tend: #08AEEA),
gamma: ( gstart: #FEE140,
gend: #FA709A,
tstart: #FA709A,
tend: #FEE140),
delta: ( gstart: #3EECAC,
gend: #EE74E1,
tstart: #EE74E1,
tend: #3EECAC));
.btn {
	outline: none;;
	margin-right: 30px;
	margin-top: 30px;
	font-family: 'Source Sans Pro', sans-serif;
	font-weight: 900;
	padding: 1.25rem 2rem;
	font-size: 1rem;
	border-radius: 3.5rem / 100%;
	position: relative;
	min-width: 15rem;
	max-width: 90vw;
	overflow: hidden;
	border: 0;
	cursor: pointer;
	text-transform: uppercase;
	letter-spacing: 0.05em;
	transition: all $time;
	outline: none;;
	&+& {
		outline: none;;
		margin-top: 1rem;
	}
	span {
		outline: none;;
		position: relative;
		z-index: 1;
	}
	&:before {
		outline: none;;
		content: "";
		background-color: #21D4FD;
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		transform: scale(4) translateX(-100%);
		transition: all $time * 1.5 ease-out;
		outline: none;;
	}
	&:hover,
	&:focus,
	&:active {
		&:before {
			outline: none;;
			transform: scale(4) translate(37%);
		}
	}
}

@each $bname,
$bcolors in $btns {
	.btn--#{$bname} {
		outline: none;;
		background-color: map-get($bcolors, gstart);
		color: map-get($bcolors, tstart);
		&:before {
			background-color: map-get($bcolors, gend);
			background-image: linear-gradient(to right, map-get($bcolors, gend) 30%, map-get($bcolors, gstart) 100%);
			position: absolute;
		}
		&:hover,
		&:focus,
		&:active {
			outline: none;;
			color: map-get($bcolors, tend);
		}
	}
}

.btn-form {
	outline: none;;
	outline: none;;
	margin-right: 30px;
	margin-top: 30px;
	font-family: 'Source Sans Pro', sans-serif;
	font-weight: 900;
	padding: 1.25rem 2rem;
	font-size: 1rem;
	border-radius: 3.5rem / 100%;
	position: relative;
	min-width: 15rem;
	max-width: 90vw;
	overflow: hidden;
	border: 0;
	cursor: pointer;
	text-transform: uppercase;
	letter-spacing: 0.05em;
	transition: all $time;
	 ;
	&+& {
		margin-top: 1rem;
		 ;
	}
	span {
		position: relative;
		z-index: 1;
		 ;
	}
	&:before {
		content: "";
		background-color: #21D4FD;
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		left: 0;
		transform: scale(4) translateX(-100%);
		transition: all $time * 1.5 ease-out;
		 ;
	}
	&:hover,
	&:focus,
	&:active {
		&:before {
			 
			//transform: scale(4) translate(37%);
		  //  color: map-get($bcolors, tstart);
		  background-color:#B721FF;;
		}
	}
}

@each $bname,
$bcolors in $btns {
	.btn-form--#{$bname} {
		background-color: map-get($bcolors, gstart);
		color: map-get($bcolors, tstart);
		&:before {
			background-color: map-get($bcolors, gend);
			background-image: linear-gradient(to right, map-get($bcolors, gend) 30%, map-get($bcolors, gstart) 100%);
			position: absolute;
		}
		&:hover,
		&:focus,
		&:active {
		//	color: map-get($bcolors, tstart);
		color: #B721FF;
		}
	}
}
/********************************h4-lists-and-contacts-information*************/

@mixin h4-list-contact-information {
    color: $black;
    font-family: AvenirNextLTProRegular;
    font-size: 0.8rem; //13px 
    line-height: 1.1rem; //18px 
    @include tablet {
        font-size: 0.9rem; //14px 
        line-height: 1.3rem; //20px
    }
    @include desktop {
        font-size: 0.9rem; //14px 
        line-height: 1.3rem; //20px 
    }
}

@mixin h4-list-contact-information-bold {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProBold;
}

@mixin h4-list-contact-information-demi {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProDemi;
}

@mixin h4-list-contact-information-white {
    @include h4-list-contact-information;
    color: $white;
}

@mixin h4-list-contact-information-white-demi {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProDemi;
    color: $white;
}

@mixin h4-list-contact-information-white-bold {
    @include h4-list-contact-information;
    font-family: AvenirNextLTProBold;
    color: $white;
}


//********************************P-Paragraph****************************/

@mixin p-paragraph {
    font-family: AvenirNextLTProRegular;
    color: $black;
    font-size: 0.8rem; //13px
    line-height: 1.1rem; //18px
    @include tablet {
        font-size: 0.9rem; //13px 
        line-height: 1.3rem; //18px 
        display: inline-block;
    }
    @include desktop {
        font-size: 0.9rem; //13px 
        line-height: 1.3rem; //18px
        display: inline-block;
    }
}

@mixin p-paragraph-bold {
    @include p-paragraph;
    font-family: AvenirNextLTProBold;
}

@mixin p-paragraph-white {
    @include p-paragraph;
    color: white;
}

@mixin p-paragraph-white-bold {
    @include p-paragraph-bold;
    color: white;
}

@mixin h1-page-header {
	font-family: TitilliumWebSemiBold600;
	color: $black;
	font-size: 28px; //28px
	line-height: 36px; //36px
	@include tablet {
        font-size: 32px; //32px
        line-height: 40px; //40px
	}
	@include desktop {
        font-size: 32px; //32px
        line-height: 40px; //40px
	}
}

@mixin h2-subheader {
	font-family: TitilliumWebSemiBold600;
	color: $black;
	font-size: 20px; //20px
	line-height: 28px; //28px
	@include tablet {
        font-size: 24px; //24px
        line-height: 32px; //28px
	}
	@include desktop {
        font-size: 24px; //24px
        line-height: 32px; //28px
	}
}
@mixin h3-labels-links-buttons {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 13px; //13px
	line-height: 20px; //20px
	@include tablet {
        font-size: 14px; //14px
        line-height: 22px; //22px
	}
	@include desktop {
        font-size: 14px; //14px
        line-height: 22px; //22px
	}
}
@mixin h4-labels-table-header {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 11px; //11px
	line-height: 16px; //16px
	@include tablet {
        font-size: 12px; //12px
        line-height: 18px; //18px
	}
	@include desktop {
        font-size: 12px; //12px
        line-height: 18px; //18px
	}
}
@mixin p1-body-large {
	font-family: TitilliumWebRegular400;
	color: $black;
    font-size: 15px; //15px
    line-height: 26px; //26px
	@include tablet {
        font-size: 16px; //12px
        line-height: 28px; //18px
	}
	@include desktop {
        font-size: 16px; //12px
        line-height: 28px; //18px
	}
}
@mixin p2-body-medium {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 16px; //13px
	line-height: 28px; //20px
	@include tablet {
        font-size: 13px; //13px
        line-height: 20px; //20px
	}
	@include desktop {
        font-size: 13px; //13px
        line-height: 20px; //20px
	}
}
@mixin p2-body-small {
	font-family: TitilliumWebRegular400;
	color: $black;
	font-size: 11px; //13px
	line-height: 16px; //20px
	@include tablet {
        font-size: 12px; //13px
        line-height: 18px; //20px
	}
	@include desktop {
        font-size: 12px; //13px
        line-height: 18px; //20px
	}
}

/*******************************************H5-Labels***********************/

@mixin h5-labels {
    font-size: 0.8rem; //12px
    line-height: 1.3rem; //20px 
    font-family: AvenirNextLTProRegular;
    color: $silver; //labels
    @include tablet {
        font-size: 0.8rem; //12px
        line-height: 1.3rem; // 20px
        font-family: AvenirNextLTProDemi;
    }
    @include desktop {
        font-size: 0.8rem; //12px
        line-height: 1.8rem; // 20px
    }
}

@mixin h5-labels-demi {
    @include h5-labels;
    font-family: AvenirNextLTProDemi;
}

@mixin h5-labels-bold {
    @include h5-labels;
    font-family: AvenirNextLTProBold;
}


//Breakpoints
@mixin tablet {
    @media (min-width: $breakpoint-tablet) {
        @content;
    }
}

@mixin desktop {
    @media (min-width: $breakpoint-desktop) {
        @content;
    }
}

//Text Styles- Desktop

@mixin d-header {
    font-family: $font-family;
    font-size: 1.875rem;
    line-height: 2.25rem;
    font-weight: 700;
	color: $text-black;
}

@mixin d-subheader {
	font-family: $font-family;
    font-size: 1.875rem;
    line-height: 2.25rem;
    font-weight: 500;
	color: $text-black;
}

@mixin d-subheader-grey {
	font-family: $font-family;
    font-size: 1.75rem;
    line-height: 2rem;
	color: $text-grey;
    font-weight: 500;
}

@mixin d-paragraph-bold {
	font-family: $font-family;
    font-size: 1.125rem;
    line-height: 1.25rem;
	color: $text-black;
    font-weight: 700;
}

@mixin d-paragraph-large {
	font-family: $font-family;
    font-size: 1.375rem;
    line-height: 1.4rem;
	color: $text-grey;
    font-weight: 400;
}

@mixin d-paragraph-medium {
	font-family: $font-family;
    font-size: 1.25rem;
    line-height: 1.4rem;
	color: $text-grey;
    font-weight: 400;
}

@mixin d-paragraph-small {
	font-family: $font-family;
    font-size: 1.4rem;
    line-height: 1rem;
	color: $text-grey;
    font-weight: 400;
}

//Button Styling
@mixin button-blue {
    background-color: $secondary-color-indigo;
    border-radius: 5px;
    color: white;
    padding: 0.875rem 1.813rem;
    appearance: none;
    border: none;
}

//Component Styling

@mixin component-styling {
    box-shadow: 0px 10px 25px rgba(19, 24, 44, 0.1);
    border-radius: 10px;
    padding: 3.75rem;
}
/***************************************STANDARD PADDING**************************************************/
@mixin standard-padding-right {
	padding-right: $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding-right: $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding-right: $mockup-column-outside-desktop; //80px right 80px left
	}
  }

  @mixin standard-padding-left {
	padding-left: $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding-left: $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding-left: $mockup-column-outside-desktop; //80px right 80px left
	}
  }
  
  @mixin standard-padding-horizontal {
	padding: 0 $mockup-column-outside-mobile; //16px right 16px left
	@include tablet {
	  padding: 0 $mockup-column-outside-tablet; //24px right 24px left
	}
	@include desktop {
	  padding: 0 $mockup-column-outside-desktop; //80px right 80px left
	}
  }
  
  
  /**********************************WIDTH*100%*PADDING**********************************************/
  
  @mixin width-100-percent-minus-standard-padding-horizontal {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent} * 2));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent} * 2));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent} * 2));
	}
  }
  
  @mixin width-100-percent-minus-standard-padding-left {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent}));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent}));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent}));
	}
  }
  
  @mixin width-100-percent-minus-standard-padding-right {
	width: calc(100% - (#{$mockup-column-outside-mobile-percent}));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet-percent}));
	}
	@include desktop {
	  width: calc(100% - (#{$mockup-column-outside-desktop-percent}));
	}
  }
  

  /*******************WIDTH MINUS MOCKUP COLUMN************/
  @mixin width-100-percent-minus-mockup-column-outside {
	width: calc(100% - #{$mockup-column-outside-mobile});
	@include tablet {
	  width: calc(100% - #{$mockup-column-outside-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-outside-desktop});
	}
  }
  @mixin width-100-percent-minus-mockup-column-outside-horizontal {
	width: calc(100% - (#{$mockup-column-outside-mobile} * 2));
	@include tablet {
	  width: calc(100% - (#{$mockup-column-outside-tablet} *2));
	}
	@include desktop {
	  width: calc(100% - (#{mockup-column-outside-desktop} *2));
	}
  }
  @mixin width-100-percent-minus-mockup-column-small {
	width: calc(100% - #{mockup-column-small-mobile});
	@include tablet {
	  width: calc(100% -#{mockup-column-small-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-small-desktop});
	}
  }
  
  @mixin width-100-percent-minus-mockup-column-large {
	width: calc(100% - #{mockup-column-large-mobile});
	@include tablet {
	  width: calc(100% -#{mockup-column-large-tablet});
	}
	@include desktop {
	  width: calc(100% - #{mockup-column-large-desktop});
	}
  }
  



@mixin fancybutton {
  font-family: 'Source Sans Pro', sans-serif;
  font-weight: 900;
  padding: 1.25rem 2rem;
  font-size: 1rem;
  border-radius: 3.5rem / 100%;
  position: relative;
  min-width: 15rem;
  max-width: 90vw;
  overflow: hidden;
  border: 0;
  cursor: pointer;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  transition: all $time;
  
  & + & {
    margin-top: 1rem;
  }
  
  span {
    position: relative;
    z-index: 1;
  }
  
  &:before {
    content: "";
    background-color: #21D4FD;
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    transform: scale(4) translateX(-100%);
    transition: all $time * 1.5 ease-out;
  }
  
  &:hover,
  &:focus,
  &:active {    
    &:before {
      transform: scale(4) translate(37%);
    } 
  }
}


  /**********************************************DIVIDERS******************************************************/
  
  @mixin divider {
	background: $silver;
	width: 100%;
	height: 1px;
  }
  
  @mixin divider-form {
	background: $silver;
	width: 100%;
	height: 1px;
	margin-bottom: $increment2-px;
  }
  
  /**********************************************LOGOS******************************************************/
  
  @mixin logo-brainflix {
	img {
	  width: 120px; //120px
	  height: 24px; //24px
	  margin-left: 1rem;
	  @include tablet {
		margin-left: 2rem;
		width: 120px; //120px
		height: 24px; //24px
	  }
	  @include desktop {
		margin-left: 5rem;
	  }
	}
  }
  
  /**********************************************BUTTONS******************************************************/
  
  @mixin button {
	background-color: $brainflix-blue;
	@include h4-paragraphs-and-comments-white-demi;
	&:hover {
	  background: $brainflix-dark-blue-hover;
	}
	&:focus {
	  outline: none;
	}
	width: $increment28-px; //226px
	height: $increment6-px; 
	border-radius: $increment0-px; 
	border-color: $brainflix-blue;
	@include tablet {
	  padding: 0;
	  width: $increment20-px; //160px
	  height: $increment6-px; 
	}
  }
  
  /**********************************************MOCKUP COLUMNS****************************************************************/
  
  /*******************RIGHT*****************/
  
  @mixin mockup-column-outside-right {
	margin-right: $mockup-column-outside-mobile;
	@include tablet {
	  margin-right: $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-right {
	margin-right: $mockup-column-small-mobile;
	@include tablet {
	  margin-right: $mockup-column-small-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-right {
	margin-right: $mockup-column-large-mobile;
	@include tablet {
	  margin-right: $mockup-column-large-tablet;
	}
	@include desktop {
	  margin-right: $mockup-column-large-desktop;
	}
  }
  
  /*******************LEFT***************/
  
  @mixin mockup-column-outside-left {
	margin-left: $mockup-column-outside-mobile;
	@include tablet {
	  margin-left: $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-left {
	margin-left: $mockup-column-small-mobile;
	@include tablet {
	  margin-left: $mockup-column-small-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-left {
	margin-left: $mockup-column-large-mobile;
	@include tablet {
	  margin-left: $mockup-column-large-tablet;
	}
	@include desktop {
	  margin-left: $mockup-column-large-desktop;
	}
  }
  
  /*********************HORIZONTAL**************/
  
  @mixin mockup-column-outside-horizontal {
	margin: 0 $mockup-column-outside-mobile;
	@include tablet {
	  margin: 0 $mockup-column-outside-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-small-horizontal {
	margin: 0 $mockup-column-small-mobile;
	@include tablet {
	  margin: 0 $mockup-column-small-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-large-horizontal {
	margin: 0 $mockup-column-large-mobile;
	@include tablet {
	  margin: 0 $mockup-column-large-tablet;
	}
	@include desktop {
	  margin: 0 $mockup-column-large-desktop;
	}
  }
  
  /*********************************************WIDTH 100% MINUS INCREMENTS*********************************/
  
  @mixin width-100-percent-minus-increment0 {
	width: calc(100% - #{increment0-mobile});
	@include tablet {
	  width: calc(100% -#{increment0-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment0-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment1 {
	width: calc(100% - #{increment1-mobile});
	@include tablet {
	  width: calc(100% - #{increment1-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment1-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment2 {
	width: calc(100% - #{increment2-mobile});
	@include tablet {
	  width: calc(100% -#{increment2-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment2-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment3 {
	width: calc(100% - #{increment3-mobile});
	@include tablet {
	  width: calc(100% -#{increment3-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment3-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment4 {
	width: calc(100% - #{increment4-mobile});
	@include tablet {
	  width: calc(100% -#{increment4-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment4-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment5 {
	width: calc(100% - #{increment5-mobile});
	@include tablet {
	  width: calc(100% -#{increment5-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment5-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment6 {
	width: calc(100% - #{increment6-mobile});
	@include tablet {
	  width: calc(100% -#{increment6-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment6-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment7 {
	width: calc(100% - #{increment7-mobile});
	@include tablet {
	  width: calc(100% -#{increment7-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment7-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment8 {
	width: calc(100% - #{increment8-mobile});
	@include tablet {
	  width: calc(100% -#{increment8-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment8-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment9 {
	width: calc(100% - #{increment9-mobile});
	@include tablet {
	  width: calc(100% -#{increment9-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment9-desktop});
	}
  }
  
  @mixin width-100-percent-minus-increment10 {
	width: calc(100% - #{increment10-mobile});
	@include tablet {
	  width: calc(100% -#{increment10-tablet});
	}
	@include desktop {
	  width: calc(100% - #{increment10-desktop});
	}
  }
  

  
  /************************************%%%%%%%%%%%%%%%%%%%%%******************************************************
  /******%%%%%%%%%%%%%%%%%%%%%%%****CONTENTS LIST:*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****TOP MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *****%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%******RIGHT MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***BOTTOM MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***LEFT MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ***********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*VERTICAL MARGIN**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ***********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*HORIZTONAL**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%***WIDTH **********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  *******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%****HEIGHT**********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****OTHER COMMON INCREMENTS*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
   ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****MOCKUP COLUMN PADDING*********%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********
  ******%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*****%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%*********/
  
  /*******************************************************INCREMENT MARGIN TOP******************************************/
  
  @mixin increment-0-top {
	margin-top: $increment0-mobile;
	@include tablet {
	  margin-top: $increment0-tablet;
	}
	@include desktop {
	  margin-top: $increment0-desktop;
	}
  }
  
  @mixin increment-1-top {
	margin-top: $increment1-mobile;
	@include tablet {
	  margin-top: $increment1-tablet;
	}
	@include desktop {
	  margin-top: $increment1-desktop;
	}
  }
  
  @mixin increment-2-top {
	margin-top: $increment2-mobile;
	@include tablet {
	  margin-top: $increment2-tablet;
	}
	@include desktop {
	  margin-top: $increment2-desktop;
	}
  }
  
  @mixin increment-3-top {
	margin-top: $increment3-mobile;
	@include tablet {
	  margin-top: $increment3-tablet;
	}
	@include desktop {
	  margin-top: $increment3-desktop;
	}
  }
  
  @mixin increment-4-top {
	margin-top: $increment4-mobile;
	@include tablet {
	  margin-top: $increment4-tablet;
	}
	@include desktop {
	  margin-top: $increment4-desktop;
	}
  }
  
  @mixin increment-5-top {
	margin-top: $increment5-mobile;
	@include tablet {
	  margin-top: $increment5-tablet;
	}
	@include desktop {
	  margin-top: $increment5-desktop;
	}
  }
  
  @mixin increment-6-top {
	margin-top: $increment6-mobile;
	@include tablet {
	  margin-top: $increment6-tablet;
	}
	@include desktop {
	  margin-top: $increment6-desktop;
	}
  }
  
  @mixin increment-7-top {
	margin-top: $increment7-mobile;
	@include tablet {
	  margin-top: $increment7-tablet;
	}
	@include desktop {
	  margin-top: $increment7-desktop;
	}
  }
  
  @mixin increment-8-top {
	margin-top: $increment8-mobile;
	@include tablet {
	  margin-top: $increment8-tablet;
	}
	@include desktop {
	  margin-top: $increment8--desktop;
	}
  }
  
  @mixin increment-9-top {
	margin-top: $increment9-mobile;
	@include tablet {
	  margin-top: $increment9-tablet;
	}
	@include desktop {
	  margin-top: $increment9--desktop;
	}
  }
  
  @mixin increment-10-top {
	margin-top: $increment10-mobile;
	@include tablet {
	  margin-top: $increment10-tablet;
	}
	@include desktop {
	  margin-top: $increment10--desktop;
	}
  }
  
  /***********************************INCREMENT RIGHT*************************************************/
  
  @mixin increment-0-right {
	margin-right: $increment0-mobile;
	@include tablet {
	  margin-right: $increment0-tablet;
	}
	@include desktop {
	  margin-right: $increment0-desktop;
	}
  }
  
  @mixin increment-1-right {
	margin-right: $increment1-mobile;
	@include tablet {
	  margin-right: $increment1-tablet;
	}
	@include desktop {
	  margin-right: $increment1-desktop;
	}
  }
  
  @mixin increment-2-right {
	margin-right: $increment2-mobile;
	@include tablet {
	  margin-right: $increment2-tablet;
	}
	@include desktop {
	  margin-right: $increment2-desktop;
	}
  }
  
  @mixin increment-3-right {
	margin-right: $increment3-mobile;
	@include tablet {
	  margin-right: $increment3-tablet;
	}
	@include desktop {
	  margin-right: $increment3-desktop;
	}
  }
  
  @mixin increment-4-right {
	margin-right: $increment4-mobile;
	@include tablet {
	  margin-right: $increment4-tablet;
	}
	@include desktop {
	  margin-right: $increment4-desktop;
	}
  }
  
  @mixin increment-5-right {
	margin-right: $increment5-mobile;
	@include tablet {
	  margin-right: $increment5-tablet;
	}
	@include desktop {
	  margin-right: $increment5-desktop;
	}
  }
  
  @mixin increment-6-right {
	margin-right: $increment6-mobile;
	@include tablet {
	  margin-right: $increment6-tablet;
	}
	@include desktop {
	  margin-right: $increment6-desktop;
	}
  }
  
  @mixin increment-7-right {
	margin-right: $increment7-mobile;
	@include tablet {
	  margin-right: $increment7-tablet;
	}
	@include desktop {
	  margin-right: $increment7-desktop;
	}
  }
  
  @mixin increment-8-right {
	margin-right: $increment8-mobile;
	@include tablet {
	  margin-right: $increment8-tablet;
	}
	@include desktop {
	  margin-right: $increment8--desktop;
	}
  }
  
  @mixin increment-9-right {
	margin-right: $increment9-mobile;
	@include tablet {
	  margin-right: $increment9-tablet;
	}
	@include desktop {
	  margin-right: $increment9--desktop;
	}
  }
  
  @mixin increment-10-right {
	margin-right: $increment10-mobile;
	@include tablet {
	  margin-right: $increment10-tablet;
	}
	@include desktop {
	  margin-right: $increment10--desktop;
	}
  }
  
  /*******************************************************INCREMENT MARGIN BOTTOM******************************************/
  
  @mixin increment-0-bottom {
	margin-bottom: $increment0-mobile;
	@include tablet {
	  margin-bottom: $increment0-tablet;
	}
	@include desktop {
	  margin-bottom: $increment0-desktop;
	}
  }
  
  @mixin increment-1-bottom {
	margin-bottom: $increment1-mobile;
	@include tablet {
	  margin-bottom: $increment1-tablet;
	}
	@include desktop {
	  margin-bottom: $increment1-desktop;
	}
  }
  
  @mixin increment-2-bottom {
	margin-bottom: $increment2-mobile;
	@include tablet {
	  margin-bottom: $increment2-tablet;
	}
	@include desktop {
	  margin-bottom: $increment2-desktop;
	}
  }
  
  @mixin increment-3-bottom {
	margin-bottom: $increment3-mobile;
	@include tablet {
	  margin-bottom: $increment3-tablet;
	}
	@include desktop {
	  margin-bottom: $increment3-desktop;
	}
  }
  
  @mixin increment-4-bottom {
	margin-bottom: $increment4-mobile;
	@include tablet {
	  margin-bottom: $increment4-tablet;
	}
	@include desktop {
	  margin-bottom: $increment4-desktop;
	}
  }
  
  @mixin increment-5-bottom {
	margin-bottom: $increment5-mobile;
	@include tablet {
	  margin-bottom: $increment5-tablet;
	}
	@include desktop {
	  margin-bottom: $increment5-desktop;
	}
  }
  
  @mixin increment-6-bottom {
	margin-bottom: $increment6-mobile;
	@include tablet {
	  margin-bottom: $increment6-tablet;
	}
	@include desktop {
	  margin-bottom: $increment6-desktop;
	}
  }
  
  @mixin increment-7-bottom {
	margin-bottom: $increment7-mobile;
	@include tablet {
	  margin-bottom: $increment7-tablet;
	}
	@include desktop {
	  margin-bottom: $increment7-desktop;
	}
  }
  
  @mixin increment-8-bottom {
	margin-bottom: $increment8-mobile;
	@include tablet {
	  margin-bottom: $increment8-tablet;
	}
	@include desktop {
	  margin-bottom: $increment8--desktop;
	}
  }
  
  @mixin increment-9-bottom {
	margin-bottom: $increment9-mobile;
	@include tablet {
	  margin-bottom: $increment9-tablet;
	}
	@include desktop {
	  margin-bottom: $increment9--desktop;
	}
  }
  
  @mixin increment-10-bottom {
	margin-bottom: $increment10-mobile;
	@include tablet {
	  margin-bottom: $increment10-tablet;
	}
	@include desktop {
	  margin-bottom: $increment10--desktop;
	}
  }
  
  /*******************************************************INCREMENT LEFT******************************************/
  
  @mixin increment-0-left {
	margin-left: $increment0-mobile;
	@include tablet {
	  margin-left: $increment0-tablet;
	}
	@include desktop {
	  margin-left: $increment0-desktop;
	}
  }
  
  @mixin increment-1-left {
	margin-left: $increment1-mobile;
	@include tablet {
	  margin-left: $increment1-tablet;
	}
	@include desktop {
	  margin-left: $increment1-desktop;
	}
  }
  
  @mixin increment-2-left {
	margin-left: $increment2-mobile;
	@include tablet {
	  margin-left: $increment2-tablet;
	}
	@include desktop {
	  margin-left: $increment2-desktop;
	}
  }
  
  @mixin increment-3-left {
	margin-left: $increment3-mobile;
	@include tablet {
	  margin-left: $increment3-tablet;
	}
	@include desktop {
	  margin-left: $increment3-desktop;
	}
  }
  
  @mixin increment-4-left {
	margin-left: $increment4-mobile;
	@include tablet {
	  margin-left: $increment4-tablet;
	}
	@include desktop {
	  margin-left: $increment4-desktop;
	}
  }
  
  @mixin increment-5-left {
	margin-left: $increment5-mobile;
	@include tablet {
	  margin-left: $increment5-tablet;
	}
	@include desktop {
	  margin-left: $increment5-desktop;
	}
  }
  
  @mixin increment-6-left {
	margin-left: $increment6-mobile;
	@include tablet {
	  margin-left: $increment6-tablet;
	}
	@include desktop {
	  margin-left: $increment6-desktop;
	}
  }
  
  @mixin increment-7-left {
	margin-left: $increment7-mobile;
	@include tablet {
	  margin-left: $increment7-tablet;
	}
	@include desktop {
	  margin-left: $increment7-desktop;
	}
  }
  
  @mixin increment-8-left {
	margin-left: $increment8-mobile;
	@include tablet {
	  margin-left: $increment8-tablet;
	}
	@include desktop {
	  margin-left: $increment8--desktop;
	}
  }
  
  @mixin increment-9-left {
	margin-left: $increment9-mobile;
	@include tablet {
	  margin-left: $increment9-tablet;
	}
	@include desktop {
	  margin-left: $increment9--desktop;
	}
  }
  
  @mixin increment-10-left {
	margin-left: $increment10-mobile;
	@include tablet {
	  margin-left: $increment10-tablet;
	}
	@include desktop {
	  margin-left: $increment10--desktop;
	}
  }
  
  /************************************VERTICAL MARGIN INCREMENTS******************************************************/
  
  @mixin increment-0-vertical {
	margin: $increment0-mobile 0;
	@include tablet {
	  margin: $increment0-tablet 0;
	}
	@include deskvertical {
	  margin: $increment0-desktop 0;
	}
  }
  
  @mixin increment-1-vertical {
	margin: $increment1-mobile 0;
	@include tablet {
	  margin: $increment1-tablet 0;
	}
	@include desktop {
	  margin: $increment1-desktop 0;
	}
  }
  
  @mixin increment-2-vertical {
	margin: $increment2-mobile 0;
	@include tablet {
	  margin: $increment2-tablet 0;
	}
	@include desktop {
	  margin: $increment2-desktop 0;
	}
  }
  
  @mixin increment-3-vertical {
	margin: $increment3-mobile 0;
	@include tablet {
	  margin: $increment3-tablet 0;
	}
	@include desktop {
	  margin: $increment3-desktop 0;
	}
  }
  
  @mixin increment-4-vertical {
	margin: $increment4-mobile 0;
	@include tablet {
	  margin: $increment4-tablet 0;
	}
	@include desktop {
	  margin: $increment4-desktop 0;
	}
  }
  
  @mixin increment-5-vertical {
	margin: $increment5-mobile 0;
	@include tablet {
	  margin: $increment5-tablet 0;
	}
	@include desktop {
	  margin: $increment5-desktop 0;
	}
  }
  
  @mixin increment-6-vertical {
	margin: $increment6-mobile 0;
	@include tablet {
	  margin: $increment6-tablet 0;
	}
	@include desktop {
	  margin: $increment6-desktop 0;
	}
  }
  
  @mixin increment-7-vertical {
	margin: $increment7-mobile 0;
	@include tablet {
	  margin: $increment7-tablet 0;
	}
	@include desktop {
	  margin: $increment7-desktop 0;
	}
  }
  
  @mixin increment-8-vertical {
	margin: $increment8-mobile 0;
	@include tablet {
	  margin: $increment8-tablet 0;
	}
	@include desktop {
	  margin: $increment8--desktop 0;
	}
  }
  
  @mixin increment-9-vertical {
	margin: $increment9-mobile 0;
	@include tablet {
	  margin: $increment9-tablet 0;
	}
	@include desktop {
	  margin: $increment9--desktop 0;
	}
  }
  
  @mixin increment-10-vertical {
	margin: $increment10-mobile 0;
	@include tablet {
	  margin: $increment10-tablet 0;
	}
	@include desktop {
	  margin: $increment10--desktop 0;
	}
  }
  
  /************************************HORIZONTAL MARGIN INCREMENTS******************************************************/
  
  @mixin increment-0-horizontal {
	margin: 0 $increment0-mobile;
	@include tablet {
	  margin: 0 $increment0-tablet;
	}
	@include deskhorizontal {
	  margin: 0 $increment0-desktop;
	}
  }
  
  @mixin increment-1-horizontal {
	margin: 0 $increment1-mobile;
	@include tablet {
	  margin: 0 $increment1-tablet;
	}
	@include desktop {
	  margin: 0 $increment1-desktop;
	}
  }
  
  @mixin increment-2-horizontal {
	margin: 0 $increment2-mobile;
	@include tablet {
	  margin: 0 $increment2-tablet;
	}
	@include desktop {
	  margin: 0 $increment2-desktop;
	}
  }
  
  @mixin increment-3-horizontal {
	margin: 0 $increment3-mobile;
	@include tablet {
	  margin: 0 $increment3-tablet;
	}
	@include desktop {
	  margin: 0 $increment3-desktop;
	}
  }
  
  @mixin increment-4-horizontal {
	margin: 0 $increment4-mobile;
	@include tablet {
	  margin: 0 $increment4-tablet;
	}
	@include desktop {
	  margin: 0 $increment4-desktop;
	}
  }
  
  @mixin increment-5-horizontal {
	margin: 0 $increment5-mobile;
	@include tablet {
	  margin: 0 $increment5-tablet;
	}
	@include desktop {
	  margin: 0 $increment5-desktop;
	}
  }
  
  @mixin increment-6-horizontal {
	margin: 0 $increment6-mobile;
	@include tablet {
	  margin: 0 $increment6-tablet;
	}
	@include desktop {
	  margin: 0 $increment6-desktop;
	}
  }
  
  @mixin increment-7-horizontal {
	margin: 0 $increment7-mobile;
	@include tablet {
	  margin: 0 $increment7-tablet;
	}
	@include desktop {
	  margin: 0 $increment7-desktop;
	}
  }
  
  @mixin increment-8-horizontal {
	margin: 0 $increment8-mobile;
	@include tablet {
	  margin: 0 $increment8-tablet;
	}
	@include desktop {
	  margin: 0 $increment8--desktop;
	}
  }
  
  @mixin increment-9-horizontal {
	margin: 0 $increment9-mobile;
	@include tablet {
	  margin: 0 $increment9-tablet;
	}
	@include desktop {
	  margin: 0 $increment9--desktop;
	}
  }
  
  @mixin increment-10-horizontal {
	margin: 0 $increment10-mobile;
	@include tablet {
	  margin: 0 $increment10-tablet;
	}
	@include desktop {
	  margin: 0 $increment10--desktop;
	}
  }
    /***************************************************INCREMENT WIDTH******************************************/
  
	@mixin increment-0-width {
		width: $increment0-mobile;
		@include tablet {
		  width: $increment0-tablet;
		}
		@include deskwidth {
		  width: $increment0-desktop;
		}
	  }
	  
	  @mixin increment-1-width {
		width: $increment1-mobile;
		@include tablet {
		  width: $increment1-tablet;
		}
		@include desktop {
		  width: $increment1-desktop;
		}
	  }
	  
	  @mixin increment-2-width {
		width: $increment2-mobile;
		@include tablet {
		  width: $increment2-tablet;
		}
		@include desktop {
		  width: $increment2-desktop;
		}
	  }
	  
	  @mixin increment-3-width {
		width: $increment3-mobile;
		@include tablet {
		  width: $increment3-tablet;
		}
		@include desktop {
		  width: $increment3-desktop;
		}
	  }
	  
	  @mixin increment-4-width {
		width: $increment4-mobile;
		@include tablet {
		  width: $increment4-tablet;
		}
		@include desktop {
		  width: $increment4-desktop;
		}
	  }
	  
	  @mixin increment-5-width {
		width: $increment5-mobile;
		@include tablet {
		  width: $increment5-tablet;
		}
		@include desktop {
		  width: $increment5-desktop;
		}
	  }
	  
	  @mixin increment-6-width {
		width: $increment6-mobile;
		@include tablet {
		  width: $increment6-tablet;
		}
		@include desktop {
		  width: $increment6-desktop;
		}
	  }
	  
	  @mixin increment-7-width {
		width: $increment7-mobile;
		@include tablet {
		  width: $increment7-tablet;
		}
		@include desktop {
		  width: $increment7-desktop;
		}
	  }
	  
	  @mixin increment-8-width {
		width: $increment8-mobile;
		@include tablet {
		  width: $increment8-tablet;
		}
		@include desktop {
		  width: $increment8--desktop;
		}
	  }
	  
	  @mixin increment-9-width {
		width: $increment9-mobile;
		@include tablet {
		  width: $increment9-tablet;
		}
		@include desktop {
		  width: $increment9--desktop;
		}
	  }
	  
	  @mixin increment-10-width {
		width: $increment10-mobile;
		@include tablet {
		  width: $increment10-tablet;
		}
		@include desktop {
		  width: $increment10--desktop;
		}
	  }
  
  /***************************************************INCREMENT HEIGHT******************************************/
  
  @mixin increment-0-height {
	height: $increment0-mobile;
	@include tablet {
	  height: $increment0-tablet;
	}
	@include deskheight {
	  height: $increment0-desktop;
	}
  }
  
  @mixin increment-1-height {
	height: $increment1-mobile;
	@include tablet {
	  height: $increment1-tablet;
	}
	@include desktop {
	  height: $increment1-desktop;
	}
  }
  
  @mixin increment-2-height {
	height: $increment2-mobile;
	@include tablet {
	  height: $increment2-tablet;
	}
	@include desktop {
	  height: $increment2-desktop;
	}
  }
  
  @mixin increment-3-height {
	height: $increment3-mobile;
	@include tablet {
	  height: $increment3-tablet;
	}
	@include desktop {
	  height: $increment3-desktop;
	}
  }
  
  @mixin increment-4-height {
	height: $increment4-mobile;
	@include tablet {
	  height: $increment4-tablet;
	}
	@include desktop {
	  height: $increment4-desktop;
	}
  }
  
  @mixin increment-5-height {
	height: $increment5-mobile;
	@include tablet {
	  height: $increment5-tablet;
	}
	@include desktop {
	  height: $increment5-desktop;
	}
  }
  
  @mixin increment-6-height {
	height: $increment6-mobile;
	@include tablet {
	  height: $increment6-tablet;
	}
	@include desktop {
	  height: $increment6-desktop;
	}
  }
  
  @mixin increment-7-height {
	height: $increment7-mobile;
	@include tablet {
	  height: $increment7-tablet;
	}
	@include desktop {
	  height: $increment7-desktop;
	}
  }
  
  @mixin increment-8-height {
	height: $increment8-mobile;
	@include tablet {
	  height: $increment8-tablet;
	}
	@include desktop {
	  height: $increment8--desktop;
	}
  }
  
  @mixin increment-9-height {
	height: $increment9-mobile;
	@include tablet {
	  height: $increment9-tablet;
	}
	@include desktop {
	  height: $increment9--desktop;
	}
  }
  
  @mixin increment-10-height {
	height: $increment10-mobile;
	@include tablet {
	  height: $increment10-tablet;
	}
	@include desktop {
	  height: $increment10--desktop;
	}
  }
  
  /***************************************************OTHER COMMON INCREMENTS********************************/
  
  /*******TOP****/
  
  @mixin increment20px-top {
	margin-top: $increment20px-mobile;
	@include tablet {
	  margin-top: $increment20px-tablet;
	}
	@include desktop {
	  margin-top: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-top {
	margin-top: $increment54px-mobile;
	@include tablet {
	  margin-top: $increment54px-tablet;
	}
	@include desktop {
	  margin-top: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-top {
	margin-top: $increment68px-mobile;
	@include tablet {
	  margin-top: $increment68px-tablet;
	}
	@include desktop {
	  margin-top: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-top {
	margin-top: $increment100px-mobile;
	@include tablet {
	  margin-top: $increment100px-tablet;
	}
	@include desktop {
	  margin-top: $increment100px-desktop;
	}
  }
  
  /***************RIGHT********************/
  
  @mixin increment20px-right {
	margin-right: $increment20px-mobile;
	@include tablet {
	  margin-right: $increment20px-tablet;
	}
	@include desktop {
	  margin-right: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-right {
	margin-right: $increment54px-mobile;
	@include tablet {
	  margin-right: $increment54px-tablet;
	}
	@include desktop {
	  margin-right: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-right {
	margin-right: $increment68px-mobile;
	@include tablet {
	  margin-right: $increment68px-tablet;
	}
	@include desktop {
	  margin-right: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-right {
	margin-right: $increment100px-mobile;
	@include tablet {
	  margin-right: $increment100px-tablet;
	}
	@include desktop {
	  margin-right: $increment100px-desktop;
	}
  }
  
  /****************BOTTOM***************/
  
  @mixin increment20px-bottom {
	margin-bottom: $increment20px-mobile;
	@include tablet {
	  margin-bottom: $increment20px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-bottom {
	margin-bottom: $increment54px-mobile;
	@include tablet {
	  margin-bottom: $increment54px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-bottom {
	margin-bottom: $increment68px-mobile;
	@include tablet {
	  margin-bottom: $increment68px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-bottom {
	margin-bottom: $increment100px-mobile;
	@include tablet {
	  margin-bottom: $increment100px-tablet;
	}
	@include desktop {
	  margin-bottom: $increment100px-desktop;
	}
  }
  
  /*****************LEFT****************/
  
  @mixin increment20px-left {
	margin-left: $increment20px-mobile;
	@include tablet {
	  margin-left: $increment20px-tablet;
	}
	@include desktop {
	  margin-left: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-left {
	margin-left: $increment54px-mobile;
	@include tablet {
	  margin-left: $increment54px-tablet;
	}
	@include desktop {
	  margin-left: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-left {
	margin-left: $increment68px-mobile;
	@include tablet {
	  margin-left: $increment68px-tablet;
	}
	@include desktop {
	  margin-left: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-left {
	margin-left: $increment100px-mobile;
	@include tablet {
	  margin-left: $increment100px-tablet;
	}
	@include desktop {
	  margin-left: $increment100px-desktop;
	}
  }
  
  /************VERTICAL***********/
  
  @mixin increment20px-vertical {
	margin: $increment20px-mobile 0;
	@include tablet {
	  margin: $increment20px-tablet 0;
	}
	@include desktop {
	  margin: $increment20px-desktop 0;
	}
  }
  
  @mixin increment54px-vertical {
	margin: $increment54px-mobile 0;
	@include tablet {
	  margin: $increment54px-tablet 0;
	}
	@include desktop {
	  margin: $increment54px-desktop 0;
	}
  }
  
  @mixin increment68px-vertical {
	margin: $increment68px-mobile 0;
	@include tablet {
	  margin: $increment68px-tablet 0;
	}
	@include desktop {
	  margin: $increment68px-desktop 0;
	}
  }
  
  @mixin increment100px-vertical {
	margin: $increment100px-mobile 0;
	@include tablet {
	  margin: $increment100px-tablet 0;
	}
	@include desktop {
	  margin: $increment100px-desktop 0;
	}
  }
  
  /**********HORIZONTAL*********/
  
  @mixin increment20px-horizontal {
	margin: 0 $increment20px-mobile;
	@include tablet {
	  margin: 0 $increment20px-tablet;
	}
	@include desktop {
	  margin: 0 $increment20px-desktop;
	}
  }
  
  @mixin increment54px-horizontal {
	margin: 0 $increment54px-mobile;
	@include tablet {
	  margin: 0 $increment54px-tablet;
	}
	@include desktop {
	  margin: 0 $increment54px-desktop;
	}
  }
  
  @mixin increment68px-horizontal {
	margin: 0 $increment68px-mobile;
	@include tablet {
	  margin: 0 $increment68px-tablet;
	}
	@include desktop {
	  margin: 0 $increment68px-desktop;
	}
  }
  
  @mixin increment100px-horizontal {
	margin: 0 $increment100px-mobile;
	@include tablet {
	  margin: 0 $increment100px-tablet;
	}
	@include desktop {
	  margin: 0 $increment100px-desktop;
	}
  }
  
 /***********WIDTH*********/
  
  @mixin increment20px-width {
	width: $increment20px-mobile;
	@include tablet {
	  width: $increment20px-tablet;
	}
	@include desktop {
	  width: $increment20px-desktop;
	}
  }

 @mixin increment54px-width {
	width: $increment54px-mobile;
	@include tablet {
	  width: $increment54px-tablet;
	}
	@include desktop {
	  width: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-width {
	width: $increment68px-mobile;
	@include tablet {
	  width: $increment68px-tablet;
	}
	@include desktop {
	  width: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-width {
	width: $increment100px-mobile;
	@include tablet {
	  width: $increment100px-tablet;
	}
	@include desktop {
	  width: $increment100px-desktop;
	}
  }

  /***********HEIGHT*********/
  @mixin increment20px-height {
	height: $increment20px-mobile;
	@include tablet {
	  height: $increment20px-tablet;
	}
	@include desktop {
	  height: $increment20px-desktop;
	}
  }
  
  @mixin increment54px-height {
	height: $increment54px-mobile;
	@include tablet {
	  height: $increment54px-tablet;
	}
	@include desktop {
	  height: $increment54px-desktop;
	}
  }
  
  @mixin increment68px-height {
	height: $increment68px-mobile;
	@include tablet {
	  height: $increment68px-tablet;
	}
	@include desktop {
	  height: $increment68px-desktop;
	}
  }
  
  @mixin increment100px-height {
	height: $increment100px-mobile;
	@include tablet {
	  height: $increment100px-tablet;
	}
	@include desktop {
	  height: $increment100px-desktop;
	}
  }
  
  /**********************************************MOCKUP COLUMNS PADDING****************************************************************/
  
  @mixin mockup-column-padding-outside-right {
	padding-right: $mockup-column-outside-mobile;
	@include tablet {
	  padding-right: $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-right {
	padding-right: $mockup-column-small-mobile;
	@include tablet {
	  padding-right: $mockup-column-small-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-right {
	padding-right: $mockup-column-large-mobile;
	@include tablet {
	  padding-right: $mockup-column-large-tablet;
	}
	@include desktop {
	  padding-right: $mockup-column-large-desktop;
	}
  }
  
  /***LEFT*******/
  
  @mixin mockup-column-padding-outside-left {
	padding-left: $mockup-column-outside-mobile;
	@include tablet {
	  padding-left: $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-left {
	padding-left: $mockup-column-small-mobile;
	@include tablet {
	  padding-left: $mockup-column-small-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-left {
	padding-left: $mockup-column-large-mobile;
	@include tablet {
	  padding-left: $mockup-column-large-tablet;
	}
	@include desktop {
	  padding-left: $mockup-column-large-desktop;
	}
  }
  
  /***HORIZONTAL*******/
  
  @mixin mockup-column-padding-outside-horizontal {
	padding: 0 $mockup-column-outside-mobile;
	@include tablet {
	  padding: 0 $mockup-column-outside-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-outside-desktop;
	}
  }
  
  @mixin mockup-column-padding-small-horizontal {
	padding: 0 $mockup-column-small-mobile;
	@include tablet {
	  padding: 0 $mockup-column-small-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-small-desktop;
	}
  }
  
  @mixin mockup-column-padding-large-horizontal {
	padding: 0 $mockup-column-large-mobile;
	@include tablet {
	  padding: 0 $mockup-column-large-tablet;
	}
	@include desktop {
	  padding: 0 $mockup-column-large-desktop;
	}
  }
  
