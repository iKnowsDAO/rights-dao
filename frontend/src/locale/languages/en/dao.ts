export default {
    proposal: "DAO Proposal",
    form: {
        title: {
            label: "Proposal Title：",
            placeholder: "Please Enter Proposal Title",
        },
        content: {
            label: "Proposal Content：",
            placeholder: "Please Enter Proposal Content",
        },
        action: {
            label: "Proposal Category：",
            placeholder: "Please Select Proposal Category",
            addAdmin: "Add Admin",
            tips: "Each Proposal can only have One Nominee"
        },
        related: {
            label: "Related People：",
            placeholder: "Please Enter the PrincipalId of the Person involved in the Proposal",
            add: "Add Proposal-Related People"
        },
        deadline: {
            tips: "Minimum Proposal Deadline is 2 Days and Maximum is 7 Days"
        }
    },
};
