//
//  CreatePostView.swift
//  VoyageAtlas
//
//  Created by Josue morales on 7/25/23.
//

import SwiftUI
import AlertToast

struct CreatePostView: View {
    @Environment(\.dismiss) var dismiss
    @State var onSuccess: () -> Void
    @State private var text = ""
    @State private var location = ""
    @State private var content = ""
    @State private var showSnackbar = false
    @State private var snackbarMsg = ""
    @State private var snackbarTitle = ""
    @State private var snackbarType: SnackbarType = SnackbarType.Error
    @StateObject private var vm = CreatePostViewModel()
    var body: some View {
        NavigationStack {
            VStack {
                GroupBox(label: Text("Create A Post")) {
                    Divider()
                    VoyageInputField(placeholder: "Title", text: $text)
                    Divider()
                    VoyageInputField(placeholder: "Location", text: $location)
                    Divider()
                    TextField("Content", text: $content, axis: .vertical)
                        .padding(8)
                        .lineLimit(5, reservesSpace: true)
                    Divider()
                    Button("Submit") {
                        snackbarTitle = "Error!"
                        if (text.isEmpty) {
                            showSnackbar = true
                            snackbarMsg = "Title is empty! Please Fill Out To Continue."
                        } else if (location.isEmpty) {
                            showSnackbar = true
                            snackbarMsg = "Location is empty! Please Fill Out To Continue."
                        } else if (content.isEmpty) {
                            showSnackbar = true
                            snackbarMsg = "Content is empty! Please Fill Out To Continue."
                        } else {
                            vm.createPost(
                                body: CreatePost(title: text, location: location, content: content),
                                onSuccess: {
                                    onSuccess()
                                    dismiss()
                                },
                                onError: {status in }
                            )
                        }
                    }
                }
                .frame(maxHeight: .infinity)
            }
            .frame(maxHeight: .infinity)
            .toast(isPresenting: $showSnackbar, duration: 2){
                AlertToast(
                    displayMode: .banner(.pop),
                    type: snackbarType == SnackbarType.Error ? .error(Color.red) : .complete(Color.green),
                    title: snackbarTitle,
                    subTitle: snackbarMsg
                )
            }
        }
        
    }
}

struct CreatePostView_Previews: PreviewProvider {
    static var previews: some View {
        CreatePostView() {
            
        }
    }
}

enum SnackbarType {
    case Error
    case Success
}
