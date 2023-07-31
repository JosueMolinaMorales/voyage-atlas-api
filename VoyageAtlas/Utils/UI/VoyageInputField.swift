//
//  VoyageInputField.swift
//  VoyageAtlas
//
//  Created by Josue morales on 7/26/23.
//

import SwiftUI

struct VoyageInputField: View {
    @State var placeholder = ""
    @State var prompt: Text?
    @State var autoCorrectDisabled = true
    @State var disabled = false
    @Binding var text: String
    @FocusState private var isFocused: Bool
    
    var body: some View {
        TextField(placeholder, text: $text, prompt: prompt)
            .padding(8)
            .autocorrectionDisabled(autoCorrectDisabled)
            .disabled(disabled)
            .focused($isFocused)
    }
}

struct VoyageInputField_Previews: PreviewProvider {
    static var previews: some View {
        @State var text = ""
        VoyageInputField(
            placeholder: "This Is a Placeholder",
            prompt: Text("Enter Something"),
            text: $text
        )
    }
}
