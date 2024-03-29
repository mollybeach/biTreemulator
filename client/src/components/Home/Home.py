@use '../../partials/variables' as *;
@use '../../partials/colors' as *;
@use '../../partials/typography' as *;
@use '../../partials/mixins' as *;
.home {
	display: flex;
	width: 100%;
	height: 90vh;
	background-color: #FFFFFF;
	@include h1-page-header;
	color: $slate;
	margin-left: 9rem;
	@include tablet{
		margin-left: 5%;
	}
	&__inside {
		padding-top: 150px;
		position: absolute;
		color: $text-grey;
		@include tablet{
			padding-top: 230px;
		}
		@include desktop{
			padding-top: 150px;
		}
	}
	&__title{
		@include h1-page-header;
		color: $raspberrypi;
		@include increment-2-bottom;
	
		font-size: 40px;
		@include tablet{
			font-size: 30px;
			//margin-left: 24px;
		}
		@include desktop{
			font-size: 30px;
			//margin-left: 80px;
		}
	}
	&__arrow {
		width: 1vw;
		padding-top: 2vw;
	}
	&__subtitle{
		margin-bottom: 1rem;
		color: $raspberrypi;
		//padding-right: 1rem;
		@include increment-2-bottom;
	    font-size: 24px;
		@include tablet{
			font-size: 45px;
		}
	}
	&__about {
		padding-top: 1%;
		padding-left: 0.1rem;
		width: 95%;
		font-size: 18px;
		color: $text-grey;
		@include tablet {
			@include h1-main-header;
			color: $text-grey;
			width: 50%;
			font-size: 36px;
		}
		@include desktop {
			@include h1-main-header;
			font-size: 45px;
			color: $text-grey;
		}
	}
}


/************************************HERO CONTAINER****************************/

.hero-container {
	display: flex;
	position: relative;
	background-image: url('../../assets/Icons/blacktree.svg');
	background-size: 150%;
	background-repeat: no-repeat;
	margin-top: 200px;
	 ::before {
		content: "";
		position: absolute;
		top: 0rem;
		right: 0rem;
		bottom: 0rem;
		left: 0rem;
		z-index: -1;
	}
	@include tablet {
		height: 100%;
		width: 100%;
	}
	@include desktop {
		margin: 10.0rem 0; //160px
  
  }

}
