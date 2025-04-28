type Social = {
  label: string;
  link: string;
};

type Presentation = {
  socials: Social[];
  profile?: string;
};

const presentation: Presentation = {
  socials: [
    {
      label: "X",
      link: "https://x.com/srn467o",
    },
    {
      label: "LinkedIn",
      link: "https://www.linkedin.com/in/srn221b/",
    },
    {
      label: "Github",
      link: "https://github.com/srn221B",
    },
  ],
};

export default presentation;
