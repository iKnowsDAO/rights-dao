export default {
    proposal: "DAO Proposal",
    create: "Create Proposal",
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
    information: {
        title: "Information",
        action: {
            label: "Proposal Category",
            add: "Add Admin",
            remove: "Remove Admin"
        },
        target: "Proposal Objectives",
        start: "Start date",
        end: "End date",
        threshold: {
            title: "Vote Threshold",
            tips: "When the number of Votes reaches a Threshold, the Proposal is automatically executed"
        },
    },
    result: {
        title: "Results",
        yes: "Yes",
        no: "No",
    },
    state: {
        open: "Open",
        accepted: "Accepted",
        rejected: "Rejected",
        executing: "Executing",
        succeeded: "Succeeded",
        failed: "Error",
    }
};
