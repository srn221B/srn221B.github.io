type Social = {
  iconName: string;
  link: string;
};

type Presentation = {
  socials: Social[];
  profile?: string;
};

const presentation: Presentation = {
  socials: [
    {
      iconName: "entypo-social:twitter",
      link: "https://x.com/srn467o",
    },
    {
      iconName: "entypo-social:linkedin",
      link: "https://www.linkedin.com/in/srn221b/",
    },
    {
      iconName: "entypo-social:github",
      link: "https://github.com/srn221B",
    },
  ],
};

export default presentation;
